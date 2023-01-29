use std::process::exit;

use crate::runtime::{PanicInfo, VirtualMachine};

pub fn default_panic_handler(info: PanicInfo) -> ! {
    match info {
        PanicInfo::Pop { vm, expected, got } => {
            println!("Virtual machine paniced at pop instruction");
            dump_vm(vm);
            println!("panic {{");
            println!("  expected: {expected}");
            println!("  got: {got}");
            println!("}}");
        }
        PanicInfo::IllegalInstruction { vm, insn } => {
            println!("Virtual machine paniced with illegal instruction");
            dump_vm(vm);
            println!("panic {{");
            println!("  insn: 0x{insn:04X}");
            println!("}}");
        }
        PanicInfo::Abort { vm } => {
            println!("Virtual machine aborted");
            dump_vm(vm);
        }
        PanicInfo::Exit { vm: _, code } => {
            exit(code as _);
        }
        PanicInfo::Panic { vm, msg } => {
            println!("Virtual machine paniced: '{}'", unsafe { (*msg).as_str() });
            dump_vm(vm);
        }
    }
    exit(-1);
}

fn dump_vm(vm: &VirtualMachine) {
    println!("vm {{");
    println!("  base_pointer: 0x{:012X}", vm.bp as usize);
    println!("  stack_pointer: 0x{:012X}", vm.sp as usize);
    println!("  program_counter: 0x{:08X}", vm.pc);
    println!("  stack_size: {}", vm.stack_size);
    println!("}}");
}
