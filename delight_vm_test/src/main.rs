use ckb_vm::{
    machine::{
        aot::AotCompilingMachine,
        asm::{AsmCoreMachine, AsmMachine},
        CoreMachine, VERSION0, VERSION1,
    },
    memory::Memory,
    registers::{A0, A1, A2, A3, A4, A5, A7},
    Debugger, DefaultMachineBuilder, Error, Instruction, Register, SupportMachine, Syscalls,
    ISA_IMC,
};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use std::{fs, u64};


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
