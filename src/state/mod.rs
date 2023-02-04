use crate::{
    error::{Error, Result},
    syntax::{CodeElement, Instruction, Span, Type, TypeElement},
    util::source::Source,
};

use self::{
    function_state::FunctionState, if_state::IfState, root_state::RootState,
    string_state::StringState, type_state::TypeState, while_state::WhileState,
};

pub mod function_state;
pub mod if_state;
pub mod loop_state;
pub mod root_state;
pub mod string_state;
pub mod type_state;
pub mod while_state;

/// Holds:
/// * the stack of all states
/// * the the last popped state to be consumed by the top state
pub struct Env<T: Source> {
    source: T,
    tmp_stack: Vec<State>,
    result: Option<State>,
}

impl<T: Source> Env<T> {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    New,
    Active,
    Waiting,
}

impl Status {
    pub fn is_new(self) -> bool {
        self == Self::New
    }

    pub fn is_active(self) -> bool {
        self == Self::Active
    }

    pub fn is_waiting(self) -> bool {
        self == Self::Waiting
    }
}

#[derive(Debug)]
pub enum State {
    Root(RootState),
    Type(TypeState),
    Function(FunctionState),
    If(IfState),
    While(WhileState),
    String(StringState),
}

impl State {
    /// Returns wether the state has finished or not
    fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        match self {
            Self::Root(it) => it.process(env),
            Self::Type(it) => it.process(env),
            Self::Function(it) => it.process(env),
            Self::If(it) => it.process(env),
            Self::While(it) => it.process(env),
            Self::String(it) => it.process(env),
        }
    }
}

pub fn process(initial_state: State, source: impl Source) -> Result<State> {
    let mut states = Vec::new();
    let mut env = Env {
        source,
        tmp_stack: Vec::with_capacity(1),
        result: None,
    };
    states.push(initial_state);
    loop {
        if !states.last_mut().unwrap().process(&mut env)? {
            states.append(&mut env.tmp_stack);
            continue;
        }
        states.append(&mut env.tmp_stack);
        if states.len() == 1 {
            return Ok(states.pop().unwrap());
        }
        env.result = Some(states.pop().unwrap());
    }
}

fn parse_buf<T: Source>(
    status: &mut Status,
    code: &mut Vec<CodeElement>,
    env: &mut Env<T>,
    index: usize,
    buf: &mut String,
) -> Result<bool> {
    if buf.contains('.') {
        if let Ok(float) = buf.parse() {
            code.push(CodeElement::Instruction(Instruction::FloatLiteral {
                span: index..env.source.index(),
                value: float,
            }));
            buf.clear();
            return Ok(false);
        };
    }
    if let Ok(int) = buf.parse() {
        code.push(CodeElement::Instruction(Instruction::IntLiteral {
            span: index..env.source.index(),
            value: int,
        }));
        buf.clear();
        return Ok(false);
    };
    match buf.as_str() {
        "type" => todo!("Implement compound types"),
        "def" => todo!("Implement functions"),
        "if" => {
            *status = Status::Waiting;
            env.tmp_stack
                .push(State::If(IfState::with_start_index(index)));
            Ok(true)
        }
        "while" => {
            *status = Status::Waiting;
            env.tmp_stack
                .push(State::While(WhileState::with_start_index(index)));
            Ok(true)
        }
        _ => {
            code.push(CodeElement::Instruction(Instruction::Call {
                span: index..env.source.index(),
            }));
            buf.clear();
            Ok(false)
        }
    }
}

fn expect_char<T: Source>(env: &mut Env<T>, c: char) -> Result<()> {
    if !env.source.has_next() {
        let index = env.source.index();
        return Err(Error::UnexpectedEndOfSource { span: index..index });
    }
    let ac = env.source.peek().unwrap();
    if c != ac {
        let index = env.source.index();
        env.source.advance();
        return Err(Error::UnexpectedCharacter {
            span: index..env.source.index(),
        });
    }
    env.source.advance();
    Ok(())
}

fn parse_type(span: Span, buf: &str) -> TypeElement {
    match buf {
        "int" => TypeElement::Type {
            span,
            value: Type::Int,
        },
        "float" => TypeElement::Type {
            span,
            value: Type::Float,
        },
        "str" => TypeElement::Type {
            span,
            value: Type::String,
        },
        _ => TypeElement::ComposeType { span },
    }
}

fn parse_types<T: Source>(env: &mut Env<T>) -> Result<Vec<TypeElement>> {
    let mut types = Vec::with_capacity(0);
    let mut buf = String::new();
    let mut index = env.source.index();
    loop {
        if !env.source.has_next() {
            let index = env.source.index();
            return Err(Error::UnexpectedEndOfSource { span: index..index });
        }
        let c = env.source.peek().unwrap();
        match c {
            ')' => {
                if !buf.is_empty() {
                    types.push(parse_type(index..env.source.index(), &buf));
                }
                break;
            }
            ',' => {
                if buf.is_empty() {
                    env.source.advance();
                    continue;
                }
                types.push(parse_type(index..env.source.index(), &buf));
                env.source.advance();
                buf.clear();
                env.source.consume_whitespace();
                index = env.source.index();
            }
            _ => {
                if c.is_whitespace() {
                    if buf.is_empty() {
                        env.source.advance();
                        continue;
                    }
                    types.push(parse_type(index..env.source.index(), &buf));
                    env.source.advance();
                    buf.clear();
                    env.source.consume_whitespace();
                    continue;
                }
                buf.push(c);
                env.source.advance();
            }
        }
    }
    Ok(types)
}

fn is_keyword(buf: &str) -> bool {
    matches!(buf, "type" | "def" | "fun" | "if" | "while")
}
