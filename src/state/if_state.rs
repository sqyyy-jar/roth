use crate::{
    error::{Error, Result},
    syntax::{CodeElement, IfStatement, Instruction, Span},
    util::source::Source,
};

use super::{string_state::StringState, Env, State, Status};

/// Holds:
/// * previous stack size
/// * `Instruction`s or `State`s (if, while) in the if element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct IfState {
    status: Status,
    start_index: usize,
    pub(super) span: Option<Span>,
    pub(super) code: Vec<CodeElement>,
}

impl IfState {
    pub fn with_start_index(start_index: usize) -> Self {
        Self {
            status: Status::New,
            start_index,
            span: None,
            code: Vec::with_capacity(0),
        }
    }

    /// Returns wether the state has finished or not
    pub fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        match self.status {
            Status::New => {
                self.status = Status::Active;
                env.source.consume_whitespace();
                if !env.source.has_next() {
                    return Err(Error::UnexpectedEndOfSource {
                        span: self.start_index..env.source.index(),
                    });
                }
                let c = env.source.peek().unwrap();
                if c != '{' {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::UnexpectedEndOfSource {
                        span: index..env.source.index(),
                    });
                }
                env.source.advance();
            }
            Status::Active => panic!("invalid status"),
            Status::Waiting => {
                let result = env.result.take().expect("result");
                match result {
                    State::Root(_) => panic!("received root state"),
                    State::Function(_) => todo!(),
                    State::If(it) => {
                        self.code.push(CodeElement::IfStatement(IfStatement {
                            span: it.span.expect("if span"),
                            code: it.code,
                        }));
                    }
                    State::While(_) => todo!(),
                    State::String(it) => {
                        self.code.push(CodeElement::Instruction(
                            it.result.expect("string state result"),
                        ));
                        if env.source.has_next() {
                            let index = env.source.index();
                            let c = env.source.peek().unwrap();
                            if !c.is_whitespace() && c != '}' {
                                env.source.advance();
                                return Err(Error::MissingWhitespaceBetweenTokens {
                                    span: index..env.source.index(),
                                });
                            }
                        }
                    }
                }
                self.status = Status::Active;
            }
        }
        let mut index = env.source.index();
        let mut buf = String::new();
        loop {
            if !env.source.has_next() || env.source.peek().unwrap().is_whitespace() {
                let was_whitespace = env.source.has_next();
                if was_whitespace {
                    env.source.consume_whitespace();
                }
                if buf.is_empty() {
                    if !env.source.has_next() {
                        break;
                    }
                    continue;
                }
                if self.parse_buf(env, index, &mut buf)? {
                    return Ok(false);
                }
            }
            let c = env.source.peek().unwrap();
            match c {
                '#' => {
                    while env.source.has_next() {
                        let c = env.source.peek().unwrap();
                        env.source.advance();
                        if c == '\n' {
                            break;
                        }
                    }
                }
                '}' => {
                    if !buf.is_empty() && self.parse_buf(env, index, &mut buf)? {
                        return Ok(false);
                    }
                    env.source.advance();
                    self.span = Some(self.start_index..index);
                    return Ok(true);
                }
                ')' | ']' => {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::ClosingBracketOutOfContext {
                        span: index..env.source.index(),
                    });
                }
                '{' => match buf.as_str() {
                    "if" => {
                        self.status = Status::Waiting;
                        env.tmp_stack
                            .push(State::If(IfState::with_start_index(index)));
                        return Ok(false);
                    }
                    "while" => todo!(),
                    _ => {
                        let index = env.source.index();
                        env.source.advance();
                        return Err(Error::OpeningBracketOutOfContext {
                            span: index..env.source.index(),
                        });
                    }
                },
                '"' => {
                    let index = env.source.index();
                    if !buf.is_empty() {
                        env.source.advance();
                        return Err(Error::MissingWhitespaceBetweenTokens {
                            span: index..env.source.index(),
                        });
                    }
                    env.source.advance();
                    self.status = Status::Waiting;
                    env.tmp_stack.push(State::String(StringState {
                        start: index,
                        value_start: env.source.index(),
                        result: None,
                    }));
                    return Ok(false);
                }
                _ => {
                    if buf.is_empty() {
                        index = env.source.index();
                    }
                    env.source.advance();
                    buf.push(c);
                }
            }
        }
        Err(Error::UnexpectedEndOfSource {
            span: index..env.source.index(),
        })
    }

    fn parse_buf<T: Source>(
        &mut self,
        env: &mut Env<T>,
        index: usize,
        buf: &mut String,
    ) -> Result<bool> {
        if buf.contains('.') {
            if let Ok(float) = buf.parse() {
                self.code
                    .push(CodeElement::Instruction(Instruction::FloatLiteral {
                        span: index..env.source.index(),
                        value: float,
                    }));
                buf.clear();
                return Ok(false);
            };
        }
        if let Ok(int) = buf.parse() {
            self.code
                .push(CodeElement::Instruction(Instruction::IntLiteral {
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
                self.status = Status::Waiting;
                env.tmp_stack
                    .push(State::If(IfState::with_start_index(index)));
                Ok(true)
            }
            "while" => todo!("Implement while"),
            _ => {
                self.code.push(CodeElement::Instruction(Instruction::Call {
                    span: index..env.source.index(),
                }));
                buf.clear();
                Ok(false)
            }
        }
    }
}
