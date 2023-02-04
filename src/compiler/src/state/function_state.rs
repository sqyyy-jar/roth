use crate::{
    error::{Error, Result},
    syntax::{CodeElement, IfStatement, Span, TypeElement, WhileStatement},
    util::source::Source,
};

use super::{
    expect_char, if_state::IfState, is_keyword, parse_buf, parse_types, string_state::StringState,
    while_state::WhileState, Env, State, Status,
};

/// Holds:
/// * the function signature
/// * `Instruction`s or `State`s (if, while) in the function element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct FunctionState {
    status: Status,
    start_index: usize,
    pub(super) span: Option<Span>,
    pub(super) name: Option<Span>,
    pub(super) input: Option<Vec<TypeElement>>,
    pub(super) output: Option<Vec<TypeElement>>,
    pub(super) code: Vec<CodeElement>,
}

impl FunctionState {
    pub fn with_start_index(start_index: usize) -> Self {
        Self {
            status: Status::New,
            start_index,
            span: None,
            name: None,
            input: None,
            output: None,
            code: Vec::with_capacity(0),
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
                        return Err(Error::UnexpectedEndOfSource { span: index..index });
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
                if is_keyword(&buf) {
                    return Err(Error::FunctionNameIsKeyword {
                        span: index..env.source.index(),
                    });
                }
                self.name = Some(index..env.source.index());
                env.source.consume_whitespace();
                expect_char(env, '(')?;
                let input = parse_types(env)?;
                env.source.consume_whitespace();
                expect_char(env, ')')?;
                env.source.consume_whitespace();
                expect_char(env, '(')?;
                let output = parse_types(env)?;
                env.source.consume_whitespace();
                expect_char(env, ')')?;
                env.source.consume_whitespace();
                self.input = Some(input);
                self.output = Some(output);
                expect_char(env, '{')?;
            }
            Status::Active => panic!("invalid status"),
            Status::Waiting => {
                let result = env.result.take().expect("result");
                match result {
                    State::Root(_) => panic!("received root state"),
                    State::Type(_) => panic!("received type state"),
                    State::Function(_) => panic!("received function state"),
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
                if buf.is_empty() {
                    if !env.source.has_next() {
                        break;
                    }
                    env.source.consume_whitespace();
                    continue;
                }
                if parse_buf(&mut self.status, &mut self.code, env, index, &mut buf)? {
                    return Ok(false);
                }
                env.source.consume_whitespace();
                continue;
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
                '(' | ')' | '[' | ']' => {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::UnexpectedCharacter {
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
