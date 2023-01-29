use std::io::{Result, Write};

use byteorder::{LittleEndian, WriteBytesExt};

use crate::{
    bytecode::*,
    parser::{Insn, PreBinary},
};

pub fn compile(write: &mut impl Write, pre_binary: &PreBinary) -> Result<()> {
    write.write_u64::<LittleEndian>(pre_binary.constants.len() as _)?;
    for constant in &pre_binary.constants {
        write.write_u64::<LittleEndian>(constant.len() as _)?;
        write.write_all(constant.as_bytes())?;
    }
    for insn in &pre_binary.instructions {
        match insn {
            Insn::Pop => write.write_u16::<LittleEndian>(INSN_POP)?,
            Insn::Ldc => write.write_u16::<LittleEndian>(INSN_LDC)?,
            Insn::Swp => write.write_u16::<LittleEndian>(INSN_SWP)?,
            Insn::Dup => write.write_u16::<LittleEndian>(INSN_DUP)?,
            Insn::Jmp => write.write_u16::<LittleEndian>(INSN_JMP)?,
            Insn::Abort => write.write_u16::<LittleEndian>(INSN_ABRT)?,
            Insn::Exit => write.write_u16::<LittleEndian>(INSN_EXIT)?,
            Insn::Panic => write.write_u16::<LittleEndian>(INSN_PANIC)?,
            Insn::Println => write.write_u16::<LittleEndian>(INSN_PRINTLN)?,
            Insn::Input => write.write_u16::<LittleEndian>(INSN_INPUT)?,
            Insn::PrintInt => write.write_u16::<LittleEndian>(INSN_PRINT_I64)?,
            Insn::PrintFloat => write.write_u16::<LittleEndian>(INSN_PRINT_F64)?,
            Insn::PrintString => write.write_u16::<LittleEndian>(INSN_PRINT_STR)?,
            Insn::AddInt => write.write_u16::<LittleEndian>(INSN_ADD_I64)?,
            Insn::AddFloat => write.write_u16::<LittleEndian>(INSN_ADD_F64)?,
            Insn::AddString => write.write_u16::<LittleEndian>(INSN_ADD_STR)?,
            Insn::SubInt => write.write_u16::<LittleEndian>(INSN_SUB_I64)?,
            Insn::SubFloat => write.write_u16::<LittleEndian>(INSN_SUB_F64)?,
            Insn::MulInt => write.write_u16::<LittleEndian>(INSN_MUL_I64)?,
            Insn::MulFloat => write.write_u16::<LittleEndian>(INSN_MUL_F64)?,
            Insn::DivInt => write.write_u16::<LittleEndian>(INSN_DIV_I64)?,
            Insn::DivFloat => write.write_u16::<LittleEndian>(INSN_DIV_F64)?,
            Insn::PushInt(value) => {
                write.write_u16::<LittleEndian>(INSN_PUSH_I64)?;
                write.write_i64::<LittleEndian>(*value)?;
            }
            Insn::PushFloat(value) => {
                write.write_u16::<LittleEndian>(INSN_PUSH_F64)?;
                write.write_f64::<LittleEndian>(*value)?;
            }
        }
    }
    Ok(())
}
