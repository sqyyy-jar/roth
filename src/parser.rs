use std::io::{Error, ErrorKind, Result};

pub enum Insn {
    Pop,
    Ldc,
    Swp,
    Dup,
    Jmp,
    Abort,
    Exit,
    Panic,
    Println,
    Input,
    PrintInt,
    PrintFloat,
    PrintString,
    AddInt,
    AddFloat,
    SubInt,
    SubFloat,
    MulInt,
    MulFloat,
    DivInt,
    DivFloat,
    PushInt(i64),
    PushFloat(f64),
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Type {
    Int,
    Float,
    String,
}

impl Type {
    fn is_int(&self) -> bool {
        *self == Type::Int
    }

    fn is_float(&self) -> bool {
        *self == Type::Float
    }

    fn is_string(&self) -> bool {
        *self == Type::String
    }
}

pub fn parse(source: &str) -> Result<Vec<Insn>> {
    let tokens = source.split_whitespace();
    let mut instructions = Vec::new();
    let mut stack = Vec::new();
    for token in tokens {
        match token {
            "+" => {
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::AddInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::AddFloat);
                    stack.push(Type::Float);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack"));
            }
            "-" => {
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::SubInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::SubFloat);
                    stack.push(Type::Float);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack"));
            }
            "*" => {
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::MulInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::MulFloat);
                    stack.push(Type::Float);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack"));
            }
            "/" => {
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::DivInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::DivFloat);
                    stack.push(Type::Float);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack"));
            }
            "pop" => {
                expect_stack_length(&stack, 1)?;
                instructions.push(Insn::Pop);
                let _ = stack.pop().unwrap();
            }
            "ldc" => {
                todo!()
            }
            "swp" => {
                expect_stack_length(&stack, 2)?;
                instructions.push(Insn::Swp);
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x);
                stack.push(y);
            }
            "dup" => {
                expect_stack_length(&stack, 1)?;
                instructions.push(Insn::Dup);
                let x = stack.pop().unwrap();
                stack.push(x);
                stack.push(x);
            }
            "jmp" => {
                expect_stack_length(&stack, 1)?;
                instructions.push(Insn::Jmp);
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "abort" => {
                instructions.push(Insn::Abort);
            }
            "exit" => {
                expect_stack_length(&stack, 1)?;
                instructions.push(Insn::Exit);
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "panic" => {
                expect_stack_length(&stack, 1)?;
                instructions.push(Insn::Panic);
                let x = stack.pop().unwrap();
                if !x.is_string() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "ln" => {
                instructions.push(Insn::Println);
            }
            "input" => {
                instructions.push(Insn::Input);
                stack.push(Type::String);
            }
            "print" => {
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                instructions.push(match x {
                    Type::Int => Insn::PrintInt,
                    Type::Float => Insn::PrintFloat,
                    Type::String => Insn::PrintString,
                });
            }
            _ => {
                if token.contains('.') {
                    if let Ok(num) = token.parse() {
                        instructions.push(Insn::PushFloat(num));
                        stack.push(Type::Float);
                        continue;
                    }
                }
                if let Ok(num) = token.parse() {
                    instructions.push(Insn::PushInt(num));
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Unknown token '{token}'"),
                ));
            }
        }
    }
    Ok(instructions)
}

fn expect_equal(x: Type, y: Type) -> Result<()> {
    if x != y {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Expected equal types on stack but found {x:?} and {y:?}"),
        ));
    }
    Ok(())
}

fn expect_stack_length(stack: &[Type], len: usize) -> Result<()> {
    if stack.len() < len {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "Expected stack with minimum length of {len}, but got length {}",
                stack.len()
            ),
        ));
    }
    Ok(())
}
