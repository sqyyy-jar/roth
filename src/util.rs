use std::{
    io::{Read, Result},
    process::exit,
};

use byteorder::{LittleEndian, ReadBytesExt};

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
        PanicInfo::InvalidConstant { vm, index } => {
            println!("Virtual machine paniced with invalid constant index {index}");
            dump_vm(vm);
        }
    }
    exit(-1);
}

pub fn read_string_constants(read: &mut impl Read) -> Result<Vec<String>> {
    let len = read.read_u64::<LittleEndian>()?;
    let mut constants = Vec::with_capacity(len as _);
    for _ in 0..len {
        let str_len = read.read_u64::<LittleEndian>()?;
        let mut buf = String::with_capacity(str_len as _);
        read.take(str_len).read_to_string(&mut buf)?;
        constants.push(buf);
    }
    Ok(constants)
}

fn dump_vm(vm: &VirtualMachine) {
    println!("vm {{");
    println!("  base_pointer: 0x{:012X}", vm.bp as usize);
    println!("  stack_pointer: 0x{:012X}", vm.sp as usize);
    println!("  program_counter: 0x{:08X}", vm.pc);
    println!("  stack_size: {}", vm.stack_size);
    println!("}}");
}
