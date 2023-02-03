use crate::{
    error::{Error, Result},
    syntax::{CodeElement, Span, TypeElement},
    util::source::Source,
};

use super::{Env, Status};

/// Holds:
/// * the function signature
/// * `Instruction`s or `State`s (if, while) in the function element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct FunctionState {
    status: Status,
    _start_index: usize,
    pub(super) _span: Option<Span>,
    pub(super) name: Option<Span>,
    pub(super) _input: Vec<TypeElement>,
    pub(super) _output: Vec<TypeElement>,
    pub(super) _code: Vec<CodeElement>,
}

impl FunctionState {
    pub fn with_start_index(start_index: usize) -> Self {
        Self {
            status: Status::New,
            _start_index: start_index,
            _span: None,
            name: None,
            _input: Vec::with_capacity(0),
            _output: Vec::with_capacity(0),
            _code: Vec::with_capacity(0),
        }
    }

    /// Returns wether the state has finished or not
    pub fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        match self.status {
            Status::New => {
                self.status = Status::Active;
                env.source.consume_whitespace();
                let index = env.source.index();
                let mut buf = String::new();
                loop {
                    if !env.source.has_next() {
                        let index = env.source.index();
                        return Err(Error::UnexpectedEndOfSource {
                            span: index..index,
                        });
                    }
                    let c = env.source.peek().unwrap();
                    match c {
                        '(' => {
                            break;
                        }
                        '[' | ']' | '{' | '}' | ')' => {
                            let index = env.source.index();
                            env.source.advance();
                            return Err(Error::UnexpectedCharacter {
                                span: index..env.source.index(),
                            });
                        }
                        '0'..='9' => {
                            env.source.advance();
                            if env.source.index() == index {
                                return Err(Error::FunctionNameStartingWithNumber {
                                    span: index..env.source.index(),
                                });
                            }
                            buf.push(c);
                        }
                        _ => {
                            if c.is_whitespace() {
                                break;
                            }
                            env.source.advance();
                            buf.push(c);
                        }
                    }
                }
                match buf.as_str() {
                    "type" | "def" | "fun" | "if" | "while" => {
                        return Err(Error::FunctionNameIsKeyword {
                            span: index..env.source.index(),
                        })
                    }
                    _ => {}
                }
                self.name = Some(index..env.source.index());
                env.source.consume_whitespace();
                expect_char(env, '(')?;
                todo!("function loading")
            }
            Status::Active => panic!("invalid status"),
            Status::Waiting => todo!(),
        }
        todo!()
    }
}

fn expect_char<T: Source>(env: &mut Env<T>, c: char) -> Result<()> {
    if !env.source.has_next() {
        let index = env.source.index();
        return Err(Error::UnexpectedEndOfSource {
            span: index..index,
        });
    }
    let ac = env.source.peek().unwrap();
    if c != ac {
        let index = env.source.index();
        env.source.advance();
        return Err(Error::UnexpectedCharacter {
            span: index..env.source.index(),
        });
    }
    Ok(())
}
