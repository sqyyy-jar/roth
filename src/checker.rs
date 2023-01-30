use std::io::{Cursor, Error, ErrorKind, Result};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::bytecode::*;

pub fn check(bytes: &[u8]) -> Result<(usize, usize)> {
    if bytes.len() % 2 != 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Instructions not aligned correctly",
        ));
    }
    let mut read = Cursor::new(bytes);
    let mut max_stack_size = 0;
    let mut stack_size = 0;
    let mut stack: Vec<Type> = Vec::new();
    while bytes.len() - read.position() as usize >= 2 {
        let insn = read.read_u16::<LittleEndian>()?;
        match insn {
            INSN_DROP => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("pop", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                stack.pop().unwrap();
            }
            INSN_LDC => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("ldc", read.position() - 2),
                    ));
                }
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("ldc", read.position() - 2),
                    ));
                }
                stack.push(Type::String);
            }
            INSN_SWP => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("swp", read.position() - 2),
                    ));
                }
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x);
                stack.push(y);
            }
            INSN_DUP => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("dup", read.position() - 2),
                    ));
                }
                stack_size += 1;
                let x = stack.pop().unwrap();
                stack.push(x);
                stack.push(x);
            }
            INSN_JMP => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("jmp", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Int, "jmp", read.position() - 2)?;
            }
            INSN_JMPIF => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("jmpif", read.position() - 2),
                    ));
                }
                stack_size -= 2;
                expect_type_on_stack(&mut stack, Type::Int, "jmpif", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Int, "jmpif", read.position() - 2)?;
            }
            INSN_JMPIFZ => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("jmpifz", read.position() - 2),
                    ));
                }
                stack_size -= 2;
                expect_type_on_stack(&mut stack, Type::Int, "jmpifz", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Int, "jmpifz", read.position() - 2)?;
            }
            INSN_PUSH_I64 => {
                stack_size += 1;
                stack.push(Type::Int);
                read.read_i64::<LittleEndian>()?;
            }
            INSN_PUSH_F64 => {
                stack_size += 1;
                stack.push(Type::Float);
                read.read_f64::<LittleEndian>()?;
            }
            INSN_NUMCONV_I64 => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("numconv-int", read.position() - 2),
                    ));
                }
                expect_type_on_stack(&mut stack, Type::Float, "numconv-int", read.position() - 2)?;
                stack.push(Type::Int);
            }
            INSN_NUMCONV_F64 => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("numconv-float", read.position() - 2),
                    ));
                }
                expect_type_on_stack(&mut stack, Type::Int, "numconv-float", read.position() - 2)?;
                stack.push(Type::Float);
            }
            INSN_ABRT => {}
            INSN_EXIT => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("exit", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Int, "exit", read.position() - 2)?;
            }
            INSN_PANIC => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("panic", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::String, "panic", read.position() - 2)?;
            }
            INSN_PRINTLN => {}
            INSN_INPUT => {
                stack_size += 1;
                stack.push(Type::String);
            }
            INSN_PRINT_I64 => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("print-int", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Int, "print-int", read.position() - 2)?;
            }
            INSN_PRINT_F64 => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("print-float", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Float, "print-float", read.position() - 2)?;
            }
            INSN_PRINT_STR => {
                if stack_size < 1 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("print-string", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(
                    &mut stack,
                    Type::String,
                    "print-string",
                    read.position() - 2,
                )?;
            }
            INSN_ADD_I64 | INSN_SUB_I64 | INSN_MUL_I64 | INSN_DIV_I64 => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("math-int", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Int, "math-int", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Int, "math-int", read.position() - 2)?;
                stack.push(Type::Int);
            }
            INSN_ADD_F64 | INSN_SUB_F64 | INSN_MUL_F64 | INSN_DIV_F64 => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("math-float", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Float, "math-float", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Float, "math-float", read.position() - 2)?;
                stack.push(Type::Float);
            }
            INSN_ADD_STR => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("add-string", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::String, "add-string", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::String, "add-string", read.position() - 2)?;
                stack.push(Type::String);
            }
            INSN_EQ_I64 | INSN_LT_I64 | INSN_GT_I64 | INSN_LE_I64 | INSN_GE_I64 => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("comp-int", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Int, "comp-int", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Int, "comp-int", read.position() - 2)?;
                stack.push(Type::Int);
            }
            INSN_EQ_F64 | INSN_LT_F64 | INSN_GT_F64 | INSN_LE_F64 | INSN_GE_F64 => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("comp-float", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::Float, "comp-float", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::Float, "comp-float", read.position() - 2)?;
                stack.push(Type::Int);
            }
            INSN_EQ_STR => {
                if stack_size < 2 {
                    return Err(Error::new(
                        ErrorKind::Other,
                        invalid_stack("comp-string", read.position() - 2),
                    ));
                }
                stack_size -= 1;
                expect_type_on_stack(&mut stack, Type::String, "comp-string", read.position() - 2)?;
                expect_type_on_stack(&mut stack, Type::String, "comp-string", read.position() - 2)?;
                stack.push(Type::Int);
            }
            _ => {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Invalid instruction 0x{insn:04X}"),
                ));
            }
        }
        if stack_size > max_stack_size {
            max_stack_size = stack_size;
        }
    }
    Ok((max_stack_size, stack_size))
}

fn expect_type_on_stack(
    stack: &mut Vec<Type>,
    type_: Type,
    insn_type: &str,
    pos: u64,
) -> Result<()> {
    let x = stack.pop().unwrap();
    if x != type_ {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid type on stack in {insn_type} instruction at position 0x{pos:08X}: expected {type_:?} but found {x:?}"),
        ));
    }
    Ok(())
}

fn invalid_stack(insn_type: &str, pos: u64) -> String {
    format!("Invalid stack at {insn_type} instruction at position 0x{pos:08X}")
}
