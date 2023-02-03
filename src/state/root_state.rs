use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    syntax::{CodeElement, ComposeType, Instruction},
    util::source::Source,
};

use super::{function_state::FunctionState, string_state::StringState, Env, State, Status};

/// Holds:
/// * `Instruction`s or `State`s (function, if, while) in the root element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct RootState {
    status: Status,
    _types: HashMap<String, ComposeType>,
    _functions: Vec<FunctionState>,
    _code: Vec<CodeElement>,
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
                    State::Function(_) => todo!(),
                    State::If(_) => todo!(),
                    State::While(_) => todo!(),
                    State::String(it) => self._code.push(CodeElement::Instruction(
                        it.result.expect("string state result"),
                    )),
                }
                self.status = Status::Active;
            }
        }
        let mut index = env.source.index();
        let mut buf = String::new();
        while env.source.has_next() {
            let c = env.source.peek().unwrap();
            if c.is_whitespace() {
                env.source.advance();
                if buf.is_empty() {
                    continue;
                }
                if buf.contains('.') {
                    if let Ok(float) = buf.parse() {
                        self._code
                            .push(CodeElement::Instruction(Instruction::FloatLiteral {
                                span: index..env.source.index(),
                                value: float,
                            }));
                        index = env.source.index();
                        buf.clear();
                        continue;
                    };
                }
                if let Ok(int) = buf.parse() {
                    self._code
                        .push(CodeElement::Instruction(Instruction::IntLiteral {
                            span: index..env.source.index(),
                            value: int,
                        }));
                    index = env.source.index();
                    buf.clear();
                    continue;
                };
                match buf.as_str() {
                    "type" => todo!("Implement compund types"),
                    "func" => {}
                    _ => {
                        self._code.push(CodeElement::Instruction(Instruction::Call {
                            span: index..env.source.index(),
                        }));
                        index = env.source.index();
                        buf.clear();
                        continue;
                    }
                }
            }
            match c {
                ')' | ']' | '}' => {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::ClosingBracketOutOfContext {
                        span: index..env.source.index(),
                    });
                }
                '"' => {
                    let index = env.source.index();
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
            _functions: Vec::with_capacity(0),
            _code: Vec::with_capacity(0),
        }
    }
}
