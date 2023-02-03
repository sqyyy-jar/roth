use crate::{
    error::Result,
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
    _status: Status,
    _span: Option<Span>,
    _name: Option<Span>,
    _input: Vec<TypeElement>,
    _output: Vec<TypeElement>,
    _code: Vec<CodeElement>,
}

impl FunctionState {
    /// Returns wether the state has finished or not
    pub fn process<T: Source>(&mut self, _env: &mut Env<T>) -> Result<bool> {
        todo!()
    }
}
