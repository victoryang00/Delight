use ckb_vm::{machine::{
    asm::{AsmCoreMachine, AsmMachine},
    CoreMachine, VERSION0,
}, memory::Memory, DefaultMachineBuilder, Error, Instruction, Register, instructions::{extract_opcode, insts}, SupportMachine, Syscalls, ISA_IMC, DefaultMachine, Machine, Bytes};
use std::{fs, path::Path, u64, io};
use ckb_vm::decoder::build_decoder;
use std::fs::DirEntry;
use std::io::ErrorKind;
use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub fn instruction_cycles(i: Instruction) -> u64 {
    match extract_opcode(i) {
        // IMC
        insts::OP_JALR => 3,
        insts::OP_LD => 2,
        insts::OP_LW => 3,
        insts::OP_LH => 3,
        insts::OP_LB => 3,
        insts::OP_LWU => 3,
        insts::OP_LHU => 3,
        insts::OP_LBU => 3,
        insts::OP_SB => 3,
        insts::OP_SH => 3,
        insts::OP_SW => 3,
        insts::OP_SD => 2,
        insts::OP_BEQ => 3,
        insts::OP_BGE => 3,
        insts::OP_BGEU => 3,
        insts::OP_BLT => 3,
        insts::OP_BLTU => 3,
        insts::OP_BNE => 3,
        insts::OP_EBREAK => 1000,
        insts::OP_ECALL => 1000,
        insts::OP_JAL => 3,
        insts::OP_MUL => 5,
        insts::OP_MULW => 5,
        insts::OP_MULH => 5,
        insts::OP_MULHU => 5,
        insts::OP_MULHSU => 5,
        insts::OP_DIV => 32,
        insts::OP_DIVW => 32,
        insts::OP_DIVU => 32,
        insts::OP_DIVUW => 32,
        insts::OP_REM => 32,
        insts::OP_REMW => 32,
        insts::OP_REMU => 32,
        insts::OP_REMUW => 32,
        // MOP
        insts::OP_WIDE_MUL => 5,
        insts::OP_WIDE_MULU => 5,
        insts::OP_WIDE_MULSU => 5,
        insts::OP_WIDE_DIV => 32,
        insts::OP_WIDE_DIVU => 32,
        insts::OP_FAR_JUMP_REL => 3,
        insts::OP_FAR_JUMP_ABS => 3,
        _ => 1,
    }
}

pub trait PProfLogger<Mac> {
    fn on_step(&mut self, machine: &mut Mac);
    fn on_exit(&mut self, machine: &mut Mac);
}

pub struct PProfMachine<'a, Inner> {
    pub machine: DefaultMachine<'a, Inner>,
    pprof_logger: Box<dyn PProfLogger<DefaultMachine<'a, Inner>>>,
}

impl<R: Register, M: Memory<REG=R>, Inner: SupportMachine<REG=R, MEM=M>> CoreMachine for PProfMachine<'_, Inner> {
    type REG = <Inner as CoreMachine>::REG;
    type MEM = <Inner as CoreMachine>::MEM;

    fn pc(&self) -> &Self::REG {
        &self.machine.pc()
    }
    fn update_pc(&mut self, pc: Self::REG) {
        self.machine.update_pc(pc)
    }
    fn commit_pc(&mut self) {
        self.machine.commit_pc()
    }
    fn memory(&self) -> &Self::MEM {
        self.machine.memory()
    }
    fn memory_mut(&mut self) -> &mut Self::MEM {
        self.machine.memory_mut()
    }
    fn registers(&self) -> &[Self::REG] {
        self.machine.registers()
    }
    fn set_register(&mut self, idx: usize, value: Self::REG) {
        self.machine.set_register(idx, value)
    }
    fn version(&self) -> u32 {
        self.machine.version()
    }
    fn isa(&self) -> u8 {
        self.machine.isa()
    }
}

impl<R: Register, M: Memory<REG=R>, Inner: SupportMachine<REG=R, MEM=M>> Machine for PProfMachine<'_, Inner> {
    fn ecall(&mut self) -> Result<(), Error> {
        self.machine.ecall()
    }
    fn ebreak(&mut self) -> Result<(), Error> {
        self.machine.ebreak()
    }
}

impl<'a, R: Register, M: Memory<REG=R>, Inner: SupportMachine<REG=R, MEM=M>> PProfMachine<'a, Inner> {
    pub fn new(
        machine: DefaultMachine<'a, Inner>,
        pprof_logger: Box<dyn PProfLogger<DefaultMachine<'a, Inner>>>,
    ) -> Self {
        Self {
            machine,
            pprof_logger,
        }
    }
    pub fn load_program(&mut self, program: &Bytes, args: &[Bytes]) -> Result<u64, Error> {
        self.machine.load_program(program, args)
    }
    pub fn run(&mut self) -> Result<i8, Error> {
        let decoder = build_decoder::<Inner::REG>(self.isa());
        self.machine.set_running(true);
        while self.machine.running() {
            self.pprof_logger.on_step(&mut self.machine);
            self.machine.step(&decoder)?;
        }
        self.pprof_logger.on_exit(&mut self.machine);
        Ok(self.machine.exit_code())
    }
}

// Use tree structure to store ckb vm's runtime data. At present, we care about cycles, but we may add other things in
// the future, for example, the number of times a certain instruction appears.
#[derive(Clone, Debug)]
struct PProfRecordTreeNode {
    name: String,
    // FILENAME + FUNCTION_NAME as expected.
    parent: Option<Rc<RefCell<PProfRecordTreeNode>>>,
    childs: Vec<Rc<RefCell<PProfRecordTreeNode>>>,
    cycles: u64,
}

impl PProfRecordTreeNode {
    // Create a new PProfRecordTreeNode with a user defined name(e.g. Function name).
    fn root() -> Self {
        Self {
            name: String::from("??:??"),
            parent: None,
            childs: vec![],
            cycles: 0,
        }
    }

    fn display_flamegraph(&self, prefix: &str, writer: &mut impl std::io::Write) {
        let prefix_name = prefix.to_owned() + self.name.as_str();
        writer
            .write_all(format!("{} {}\n", prefix_name, self.cycles).as_bytes())
            .unwrap();
        for e in &self.childs {
            e.borrow_mut().display_flamegraph(&(prefix_name.as_str().to_owned() + "; "), writer);
        }
    }
}

struct PProfLogger_eval {
    atsl_context: addr2line::Context<
        addr2line::gimli::EndianReader<addr2line::gimli::RunTimeEndian, std::rc::Rc<[u8]>>,
    >,
    tree_root: Rc<RefCell<PProfRecordTreeNode>>,
    tree_node: Rc<RefCell<PProfRecordTreeNode>>,
    ra_dict: HashMap<u64, Rc<RefCell<PProfRecordTreeNode>>>,
}

impl PProfLogger_eval {
    fn new(filename: String) -> Result<Self, Box<dyn std::error::Error>> {
        let file = std::fs::File::open(filename)?;
        let mmap = unsafe { memmap::Mmap::map(&file)? };
        let object = object::File::parse(&*mmap)?;
        let ctx = addr2line::Context::new(&object)?;
        let tree_root = Rc::new(RefCell::new(PProfRecordTreeNode::root()));
        Ok(Self {
            atsl_context: ctx,
            tree_root: tree_root.clone(),
            tree_node: tree_root,
            ra_dict: HashMap::new(),
        })
    }
}

fn print_dir_contents(dir: &Path) -> Result<String, Box<io::Error>> {
    if !dir.is_dir() {
        return Ok("is not a directory!".parse().unwrap());
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        let file_name = path.file_name().unwrap();
        if file_name.to_string_lossy().contains("delight_book") {
            return Ok(file_name.to_string_lossy().parse().unwrap());
        }
    }

    return Ok("Not found".parse().unwrap());
}
#[allow(dead_code)]
fn sprint_loc_file_line(loc: &Option<addr2line::Location>) -> String {
    if let Some(ref loc) = *loc {
        let mut list = vec![];
        let file = loc.file.as_ref().unwrap();
        let path = Path::new(file);
        list.push(path.display().to_string());
        if let Some(line) = loc.line {
            list.push(format!("{}", line));
        } else {
            list.push(String::from("??"));
        }
        list.join(":")
    } else {
        String::from("??:??")
    }
}

fn sprint_loc_file(loc: &Option<addr2line::Location>) -> String {
    if let Some(ref loc) = *loc {
        let file = loc.file.as_ref().unwrap();
        let path = Path::new(file);
        path.display().to_string()
    } else {
        String::from("??")
    }
}
#[allow(dead_code)]
fn sprint_fun(
    mut frame_iter: addr2line::FrameIter<
        addr2line::gimli::EndianReader<addr2line::gimli::RunTimeEndian, std::rc::Rc<[u8]>>,
    >,
) -> String {
    let mut stack: Vec<String> = vec![String::from("??")];
    loop {
        let frame = frame_iter.next().unwrap();
        if frame.is_none() {
            break;
        }
        let frame = frame.unwrap();
        let function = frame.function.unwrap();
        let function_name = String::from(addr2line::demangle_auto(
            Cow::from(function.raw_name().unwrap()),
            function.language,
        ));

        stack.push(function_name)
    }
    stack.last().unwrap().to_string()
}

impl<'a, R: Register, M: Memory<REG = R>, Inner: ckb_vm::machine::SupportMachine<REG = R, MEM = M>> PProfLogger<ckb_vm::machine::DefaultMachine<'a, Inner>> for PProfLogger_eval {
    fn on_step(&mut self, machine: &mut ckb_vm::machine::DefaultMachine<'a, Inner>) {
        let pc = machine.pc().to_u64();
        let decoder = ckb_vm::decoder::build_decoder::<R>(machine.isa());
        let inst = decoder.decode(machine.memory_mut(), pc).unwrap();
        let opcode = ckb_vm::instructions::extract_opcode(inst);
        let cycles = machine
            .instruction_cycle_func()
            .as_ref()
            .map(|f| f(inst))
            .unwrap_or(0);

        if opcode == ckb_vm::instructions::insts::OP_JAL {
            let inst = ckb_vm::instructions::Utype(inst);
            if inst.rd() == ckb_vm::registers::RA {
                let d = pc.overflowing_add(inst.immediate_s() as u64).0 & 0xfffffffffffffffe;
                let loc = self.atsl_context.find_location(d).unwrap();
                let loc_string = sprint_loc_file(&loc);
                let frame_iter = self.atsl_context.find_frames(d).unwrap();
                let fun_string = sprint_fun(frame_iter);
                let tag_string = format!("{}:{}", loc_string, fun_string);
                let chd = Rc::new(RefCell::new(PProfRecordTreeNode {
                    name: tag_string,
                    parent: Some(self.tree_node.clone()),
                    childs: vec![],
                    cycles: 0,
                }));
                self.tree_node.borrow_mut().childs.push(chd.clone());
                self.ra_dict.insert(pc + 4, self.tree_node.clone());
                self.tree_node = chd;
            }
        };
        if opcode == ckb_vm::instructions::insts::OP_JALR {
            let inst = ckb_vm::instructions::Itype(inst);
            if inst.rd() == ckb_vm::registers::RA {
                let d = machine.registers()[inst.rs1()]
                    .to_u64()
                    .overflowing_add(inst.immediate_s() as u64)
                    .0
                    & 0xfffffffffffffffe;
                let loc = self.atsl_context.find_location(d).unwrap();
                let loc_string = sprint_loc_file(&loc);
                let frame_iter = self.atsl_context.find_frames(d).unwrap();
                let fun_string = sprint_fun(frame_iter);
                let tag_string = format!("{}:{}", loc_string, fun_string);
                let chd = Rc::new(RefCell::new(PProfRecordTreeNode {
                    name: tag_string,
                    parent: Some(self.tree_node.clone()),
                    childs: vec![],
                    cycles: 0,
                }));
                self.tree_node.borrow_mut().childs.push(chd.clone());
                self.ra_dict.insert(pc + 4, self.tree_node.clone());
                self.tree_node = chd;
            }
        };
        self.tree_node.borrow_mut().cycles += cycles;
    }

    fn on_exit(&mut self, machine: &mut ckb_vm::machine::DefaultMachine<'a, Inner>) {
        assert_eq!(machine.exit_code(), 0);
        self.tree_root
            .borrow_mut()
            .display_flamegraph("", &mut std::io::stdout());
    }
}

fn main() {
    match print_dir_contents(Path::new("target/riscv64imac-unknown-none-elf/debug/deps/")) {
        Ok(s) => {
            let path = "target/riscv64imac-unknown-none-elf/debug/deps/".to_owned() + &s;
            let buffer =
                fs::read(path.clone())
                    .unwrap()
                    .into();
            println!("Dir: {}", s.to_string());
            let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
            let core = DefaultMachineBuilder::new(asm_core).build();
            let mut machine = AsmMachine::new(core, None);
            machine
                .load_program(&buffer, &vec!["simple".into()])
                .unwrap();
            machine.run();
            // Second run from https://github.com/nervosnetwork/ckb-vm-pprof/blob/596d33aff44259d4013968d95f5f8d6147cd496f/src/main.rs
            let code = fs::read(path.clone()).unwrap();
            let code_data = Bytes::from(code);
            let default_core_machine = ckb_vm::DefaultCoreMachine::<u64, ckb_vm::SparseMemory<u64>>::new(
                ckb_vm::ISA_IMC | ckb_vm::ISA_B,
                ckb_vm::machine::VERSION1,
                1 << 32,
            );
            let default_machine_builder = ckb_vm::DefaultMachineBuilder::new(default_core_machine)
                .instruction_cycle_func(Box::new(instruction_cycles));
            let default_machine = default_machine_builder.build();
            let prof = Box::new(PProfLogger_eval::new(path).unwrap());
            let mut machine = PProfMachine::new(default_machine, prof);
            let mut args = vec![("target/riscv64imac-unknown-none-elf/debug/deps/".to_owned() + &s).to_string().into()];
            machine.load_program(&code_data, &args).unwrap();
            machine.run();
        }
        Err(s) => println!("Error: {}", s.to_string()),
    }
}
