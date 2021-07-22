use ckb_vm::{machine::{
    aot::AotCompilingMachine,
    asm::{AsmCoreMachine, AsmMachine},
    CoreMachine, VERSION0, VERSION1,
}, memory::Memory, Debugger, DefaultMachineBuilder, Error, Instruction, Register, instructions::{extract_opcode, insts}, SupportMachine, Syscalls, ISA_IMC, DefaultMachine, Machine, Bytes};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::{fs, u64};
use std::fmt::Debug;
use ckb_vm::decoder::build_decoder;

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
        insts::OP_EBREAK => 500,
        insts::OP_ECALL => 500,
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

fn sprint_loc_file_line(loc:&Option<addr2line::Location>)->String{
    todo!()
}

fn sprint_loc_file(loc:&Option<addr2line::Location>)->String{
    todo!()
}

impl<R: Register, M: Memory<REG=R>, Inner: SupportMachine<REG=R, MEM=M>> Machine
for PProfMachine<'_, Inner>
{
    fn ecall(&mut self) -> Result<(), Error> {
        self.machine.ecall()
    }
    fn ebreak(&mut self) -> Result<(), Error> {
        self.machine.ebreak()
    }
}

impl<'a, R: Register, M: Memory<REG=R>, Inner: SupportMachine<REG=R, MEM=M>>
PProfMachine<'a, Inner>
{
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

fn main() {
    let buffer =
        fs::read("target/riscv64imac-unknown-none-elf/debug/deps/delight_book-29257b8d53a613ae")
            .unwrap()
            .into();
    let asm_core = AsmCoreMachine::new(ISA_IMC, VERSION0, u64::max_value());
    let core = DefaultMachineBuilder::new(asm_core).build();
    let mut machine = AsmMachine::new(core, None);
    machine
        .load_program(&buffer, &vec!["simple".into()])
        .unwrap();
    let result = machine.run();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}
