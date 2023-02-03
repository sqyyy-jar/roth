use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    syntax::{CodeElement, ComposeType, IfStatement, Instruction, WhileStatement},
    util::source::Source,
};

use super::{
    function_state::FunctionState, if_state::IfState, string_state::StringState,
    while_state::WhileState, Env, State, Status,
};

/// Holds:
/// * `Instruction`s or `State`s (function, if, while) in the root element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct RootState {
    status: Status,
    _types: HashMap<String, ComposeType>,
    functions: Vec<FunctionState>,
    code: Vec<CodeElement>,
}

impl RootState {
    /// Returns wether the state has finished or not
    pub(super) fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        match self.status {
            Status::New => {
                self.status = Status::Active;
            }
            Status::Active => panic!("invalid status"),
            Status::Waiting => {
                let result = env.result.take().expect("result");
                match result {
                    State::Root(_) => panic!("received root state"),
                    State::Function(it) => {
                        self.functions.push(it);
                    }
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
                            if !c.is_whitespace() {
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
                if buf.contains('.') {
                    if let Ok(float) = buf.parse() {
                        self.code
                            .push(CodeElement::Instruction(Instruction::FloatLiteral {
                                span: index..env.source.index(),
                                value: float,
                            }));
                        buf.clear();
                        continue;
                    };
                }
                if let Ok(int) = buf.parse() {
                    self.code
                        .push(CodeElement::Instruction(Instruction::IntLiteral {
                            span: index..env.source.index(),
                            value: int,
                        }));
                    buf.clear();
                    continue;
                };
                match buf.as_str() {
                    "type" => todo!("Implement compound types"),
                    "def" | "fun" => {
                        self.status = Status::Waiting;
                        env.tmp_stack
                            .push(State::Function(FunctionState::with_start_index(index)));
                        return Ok(false);
                    }
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
                        self.code.push(CodeElement::Instruction(Instruction::Call {
                            span: index..env.source.index(),
                        }));
                        buf.clear();
                        continue;
                    }
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
                ')' | ']' | '}' => {
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
        Ok(true)
    }
}

impl Default for RootState {
    fn default() -> Self {
        Self {
            status: Status::New,
            _types: HashMap::with_capacity(0),
            functions: Vec::with_capacity(0),
            code: Vec::with_capacity(0),
        }
    }
}
