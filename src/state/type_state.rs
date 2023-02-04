use crate::{
    error::{Error, Result},
    syntax::{Span, TypeElement},
    util::source::Source,
};

use super::{expect_char, is_keyword, parse_types, Env};

#[derive(Debug)]
pub struct TypeState {
    pub(super) start_index: usize,
    pub(super) span: Option<Span>,
    pub(super) name: Option<Span>,
    pub(super) types: Option<Vec<TypeElement>>,
}

impl TypeState {
    pub fn with_start_index(start_index: usize) -> Self {
        Self {
            start_index,
            span: None,
            name: None,
            types: None,
        }
    }

    pub fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
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
        let types = parse_types(env)?;
        env.source.consume_whitespace();
        expect_char(env, ')')?;
        self.span = Some(self.start_index..env.source.index());
        self.types = Some(types);
        Ok(true)
    }
}
