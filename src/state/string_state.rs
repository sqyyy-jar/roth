use crate::{
    error::{Error, Result},
    syntax::Instruction,
    util::source::Source,
};

use super::Env;

#[derive(Debug)]
pub struct StringState {
    pub(super) start: usize,
    pub(super) value_start: usize,
    pub(super) result: Option<Instruction>,
}

impl StringState {
    /// Returns wether the state has finished or not
    pub(super) fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        while env.source.has_next() {
            let c = env.source.peek().unwrap();
            match c {
                '\n' => {
                    let index = env.source.index();
                    env.source.advance();
                    return Err(Error::NewlineInStringLiteral {
                        span: index..env.source.index(),
                    });
                }
                '"' => {
                    let index = env.source.index();
                    env.source.advance();
                    self.result = Some(Instruction::StringLiteral {
                        span: self.start..env.source.index(),
                        value: self.value_start..index,
                    });
                    return Ok(true);
                }
                '\\' => {
                    let index = env.source.index();
                    env.source.advance();
                    if !env.source.has_next() {
                        return Err(Error::UnexpectedEndOfSource {
                            span: self.start..env.source.index(),
                        });
                    }
                    let ec = env.source.peek().unwrap();
                    env.source.advance();
                    match ec {
                        'n' | 'r' | 't' | '"' | '\\' => {}
                        _ => {
                            return Err(Error::InvalidEscapeCharacterInString {
                                span: index..env.source.index(),
                            });
                        }
                    }
                }
                _ => env.source.advance(),
            }
        }
        Err(Error::UnexpectedEndOfSource {
            span: self.start..env.source.index(),
        })
    }
}
