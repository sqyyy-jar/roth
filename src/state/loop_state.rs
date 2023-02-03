use crate::{
    error::{Error, Result},
    syntax::{CodeElement, IfStatement, Span, WhileStatement},
    util::source::Source,
};

use super::{
    if_state::IfState, parse_buf, string_state::StringState, while_state::WhileState, Env, State,
    Status,
};

/// Holds:
/// * previous stack size
/// * `Instruction`s or `State`s (if, while) in the loop element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct LoopState {
    status: Status,
    start_index: usize,
    pub(super) span: Option<Span>,
    pub(super) code: Vec<CodeElement>,
}

impl LoopState {
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
                    let index = env.source.index();
                    return Err(Error::UnexpectedEndOfSource { span: index..index });
                }
                let c = env.source.peek().unwrap();
                if c != '{' {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::UnexpectedCharacter {
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
                            span: it.inner.span.expect("if span"),
                            code: it.inner.code,
                        }));
                    }
                    State::While(it) => {
                        self.code.push(CodeElement::WhileStatement(WhileStatement {
                            span: it.inner.span.expect("while span"),
                            code: it.inner.code,
                        }));
                    }
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
                if parse_buf(&mut self.status, &mut self.code, env, index, &mut buf)? {
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
                    if !buf.is_empty()
                        && parse_buf(&mut self.status, &mut self.code, env, index, &mut buf)?
                    {
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
                    "while" => {
                        self.status = Status::Waiting;
                        env.tmp_stack
                            .push(State::While(WhileState::with_start_index(index)));
                        return Ok(false);
                    }
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
        let index = env.source.index();
        Err(Error::UnexpectedEndOfSource { span: index..index })
    }
}
