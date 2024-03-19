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
            Insn::Drop => write.write_u16::<LittleEndian>(INSN_DROP)?,
            Insn::Load => write.write_u16::<LittleEndian>(INSN_LOAD)?,
            Insn::Swap => write.write_u16::<LittleEndian>(INSN_SWAP)?,
            Insn::Dup => write.write_u16::<LittleEndian>(INSN_DUP)?,
            Insn::Jump => write.write_u16::<LittleEndian>(INSN_J)?,
            Insn::JumpNotZero => write.write_u16::<LittleEndian>(INSN_JNZ)?,
            Insn::JumpZero => write.write_u16::<LittleEndian>(INSN_JZ)?,
            Insn::PushInt(value) => {
                write.write_u16::<LittleEndian>(INSN_PUSH_I64)?;
                write.write_i64::<LittleEndian>(*value)?;
            }
            Insn::PushFloat(value) => {
                write.write_u16::<LittleEndian>(INSN_PUSH_F64)?;
                write.write_f64::<LittleEndian>(*value)?;
            }
            Insn::NumConvInt => write.write_u16::<LittleEndian>(INSN_NUMCONV_I64)?,
            Insn::NumConvFloat => write.write_u16::<LittleEndian>(INSN_NUMCONV_F64)?,
            Insn::TriRot => write.write_u16::<LittleEndian>(INSN_TROT)?,
            Insn::DiDup => write.write_u16::<LittleEndian>(INSN_DDUP)?,
            Insn::TriDup => write.write_u16::<LittleEndian>(INSN_TDUP)?,
            Insn::Abort => write.write_u16::<LittleEndian>(INSN_ABORT)?,
            Insn::Exit => write.write_u16::<LittleEndian>(INSN_EXIT)?,
            Insn::Panic => write.write_u16::<LittleEndian>(INSN_PANIC)?,
            Insn::Println => write.write_u16::<LittleEndian>(INSN_PRINTLN)?,
            Insn::Input => write.write_u16::<LittleEndian>(INSN_INPUT)?,
            Insn::Gc => write.write_u16::<LittleEndian>(INSN_GC)?,
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
            Insn::EqInt => write.write_u16::<LittleEndian>(INSN_EQ_I64)?,
            Insn::LtInt => write.write_u16::<LittleEndian>(INSN_LT_I64)?,
            Insn::GtInt => write.write_u16::<LittleEndian>(INSN_GT_I64)?,
            Insn::LeInt => write.write_u16::<LittleEndian>(INSN_LE_I64)?,
            Insn::GeInt => write.write_u16::<LittleEndian>(INSN_GE_I64)?,
            Insn::EqFloat => write.write_u16::<LittleEndian>(INSN_EQ_F64)?,
            Insn::LtFloat => write.write_u16::<LittleEndian>(INSN_LT_F64)?,
            Insn::GtFloat => write.write_u16::<LittleEndian>(INSN_GT_F64)?,
            Insn::LeFloat => write.write_u16::<LittleEndian>(INSN_LE_F64)?,
            Insn::GeFloat => write.write_u16::<LittleEndian>(INSN_GE_F64)?,
            Insn::EqString => write.write_u16::<LittleEndian>(INSN_EQ_STR)?,
        }
    }
    Ok(())
}
