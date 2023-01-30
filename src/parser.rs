use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Result},
};

use crate::{bytecode::Type, Flags};

pub struct PreBinary {
    pub constants: Vec<String>,
    pub instructions: Vec<Insn>,
}

pub enum Insn {
    Drop,
    Ldc,
    Swp,
    Dup,
    Jmp,
    JmpIf,
    JmpIfZ,
    PushInt(i64),
    PushFloat(f64),
    NumConvInt,
    NumConvFloat,
    Abort,
    Exit,
    Panic,
    Println,
    Input,
    PrintInt,
    PrintFloat,
    PrintString,
    AddInt,
    SubInt,
    MulInt,
    DivInt,
    AddFloat,
    SubFloat,
    MulFloat,
    DivFloat,
    AddString,
    EqInt,
    LtInt,
    GtInt,
    LeInt,
    GeInt,
    EqFloat,
    LtFloat,
    GtFloat,
    LeFloat,
    GeFloat,
    EqString,
}

pub fn parse(source: &str, flags: &Flags) -> Result<PreBinary> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let pre_code = source.replace('\r', "");
    let mut chars = pre_code.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '#' => {
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
                for ac in chars.by_ref() {
                    if ac == '\n' {
                        break;
                    }
                }
            }
            '"' => {
                token.push(c);
                while let Some(ac) = chars.next() {
                    if ac == '"' {
                        token.push(ac);
                        break;
                    }
                    if ac == '\n' {
                        return Err(Error::new(ErrorKind::Other, "Invalid string literal"));
                    }
                    if ac == '\\' {
                        let Some(ac) = chars.next() else {
                            return Err(Error::new(ErrorKind::Other, "Invalid escape in string"));
                        };
                        match ac {
                            '"' => token.push('"'),
                            'n' => token.push('\n'),
                            'r' => token.push('\r'),
                            't' => token.push('\t'),
                            _ => {
                                return Err(Error::new(
                                    ErrorKind::Other,
                                    "Invalid escape in string",
                                ));
                            }
                        }
                        continue;
                    }
                    token.push(ac);
                }
                tokens.push(token);
                token = String::new();
            }
            _ => {
                if c.is_whitespace() {
                    if !token.is_empty() {
                        tokens.push(token);
                        token = String::new();
                    }
                    while let Some(ac) = chars.peek() {
                        if !ac.is_whitespace() {
                            break;
                        }
                        chars.next().unwrap();
                    }
                    continue;
                }
                token.push(c);
                while let Some(ac) = chars.peek().cloned() {
                    if ac.is_whitespace() {
                        break;
                    }
                    chars.next().unwrap();
                    token.push(ac);
                }
                if !token.is_empty() {
                    tokens.push(token);
                    token = String::new();
                }
            }
        }
    }
    let mut instructions = Vec::new();
    let mut stack = Vec::new();
    let mut byte_index = 0;
    let mut labels = HashMap::new();
    let mut constants = Vec::new();
    for token in tokens {
        match token.as_str() {
            "+" => {
                byte_index += 2;
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
                if x.is_string() {
                    instructions.push(Insn::AddString);
                    stack.push(Type::String);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to add"));
            }
            "-" => {
                byte_index += 2;
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
                return Err(Error::new(ErrorKind::Other, "Invalid stack to subtract"));
            }
            "*" => {
                byte_index += 2;
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
                return Err(Error::new(ErrorKind::Other, "Invalid stack to multiply"));
            }
            "/" => {
                byte_index += 2;
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
                return Err(Error::new(ErrorKind::Other, "Invalid stack to divide"));
            }
            "=" => {
                byte_index += 2;
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::EqInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::EqFloat);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_string() {
                    instructions.push(Insn::EqString);
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to compare"));
            }
            "<" => {
                byte_index += 2;
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::LtInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::LtFloat);
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to compare"));
            }
            ">" => {
                byte_index += 2;
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::GtInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::GtFloat);
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to compare"));
            }
            "<=" => {
                byte_index += 2;
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::LeInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::LeFloat);
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to compare"));
            }
            ">=" => {
                byte_index += 2;
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                expect_equal(x, y)?;
                if x.is_int() {
                    instructions.push(Insn::GeInt);
                    stack.push(Type::Int);
                    continue;
                }
                if x.is_float() {
                    instructions.push(Insn::GeFloat);
                    stack.push(Type::Int);
                    continue;
                }
                return Err(Error::new(ErrorKind::Other, "Invalid stack to compare"));
            }
            "drop" => {
                byte_index += 2;
                instructions.push(Insn::Drop);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let _ = stack.pop().unwrap();
            }
            "ldc" => {
                byte_index += 2;
                instructions.push(Insn::Ldc);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Invalid stack to load constant",
                    ));
                }
            }
            "swp" => {
                byte_index += 2;
                instructions.push(Insn::Swp);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                stack.push(x);
                stack.push(y);
            }
            "dup" => {
                byte_index += 2;
                instructions.push(Insn::Dup);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                stack.push(x);
                stack.push(x);
            }
            "if" => {
                byte_index += 2;
                instructions.push(Insn::JmpIf);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                if !x.is_int() || !y.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "!if" => {
                byte_index += 2;
                instructions.push(Insn::JmpIfZ);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 2)?;
                let x = stack.pop().unwrap();
                let y = stack.pop().unwrap();
                if !x.is_int() || !y.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "abort" => {
                byte_index += 2;
                instructions.push(Insn::Abort);
            }
            "exit" => {
                byte_index += 2;
                instructions.push(Insn::Exit);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "panic" => {
                byte_index += 2;
                instructions.push(Insn::Panic);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                if !x.is_string() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
            }
            "ln" => {
                byte_index += 2;
                instructions.push(Insn::Println);
            }
            "input" => {
                byte_index += 2;
                instructions.push(Insn::Input);
                if !flags.verify {
                    continue;
                }
                stack.push(Type::String);
            }
            "print" => {
                byte_index += 2;
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                instructions.push(match x {
                    Type::Int => Insn::PrintInt,
                    Type::Float => Insn::PrintFloat,
                    Type::String => Insn::PrintString,
                });
            }
            "~float" => {
                byte_index += 2;
                instructions.push(Insn::NumConvFloat);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                if !x.is_int() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
                stack.push(Type::Float);
            }
            "~int" => {
                byte_index += 2;
                instructions.push(Insn::NumConvInt);
                if !flags.verify {
                    continue;
                }
                expect_stack_length(&stack, 1)?;
                let x = stack.pop().unwrap();
                if !x.is_float() {
                    return Err(Error::new(ErrorKind::Other, "Invalid stack"));
                }
                stack.push(Type::Int);
            }
            "%int" => {
                if flags.verify {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Feature only available in noverify mode",
                    ));
                }
                stack.push(Type::Int);
            }
            "%float" => {
                if flags.verify {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Feature only available in noverify mode",
                    ));
                }
                stack.push(Type::Float);
            }
            "%str" => {
                if flags.verify {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Feature only available in noverify mode",
                    ));
                }
                stack.push(Type::String);
            }
            _ => {
                if let Some(label) = token.strip_prefix(':') {
                    labels.insert(label.to_string(), byte_index);
                    continue;
                }
                if let Some(label) = token.strip_prefix('@') {
                    let Some(index) = labels.get(label) else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Unknown label '{label}'"),
                        ));
                    };
                    byte_index += 10 + 2;
                    instructions.push(Insn::PushInt(*index));
                    instructions.push(Insn::Jmp);
                    continue;
                }
                if let Some(label) = token.strip_prefix('&') {
                    let Some(index) = labels.get(label) else {
                        return Err(Error::new(
                            ErrorKind::Other,
                            format!("Unknown label '{label}'"),
                        ));
                    };
                    byte_index += 10;
                    instructions.push(Insn::PushInt(*index));
                    stack.push(Type::Int);
                    continue;
                }
                if let Some(string) = token.strip_prefix('"') {
                    byte_index += 10 + 2;
                    instructions.push(Insn::PushInt(constants.len() as _));
                    instructions.push(Insn::Ldc);
                    constants.push(string[0..string.len() - 1].to_string());
                    stack.push(Type::String);
                    continue;
                }
                if token.contains('.') {
                    if let Ok(num) = token.parse() {
                        byte_index += 10;
                        instructions.push(Insn::PushFloat(num));
                        stack.push(Type::Float);
                        continue;
                    }
                }
                if let Ok(num) = token.parse() {
                    byte_index += 10;
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
    Ok(PreBinary {
        constants,
        instructions,
    })
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
