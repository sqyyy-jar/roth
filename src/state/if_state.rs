use crate::{
    error::Result,
    syntax::{CodeElement, Span},
    util::source::Source,
};

use super::Env;

/// Holds:
/// * previous stack size
/// * `Instruction`s or `State`s (if, while) in the if element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct IfState {
    _span: Option<Span>,
    _code: Vec<CodeElement>,
}

impl IfState {
    /// Returns wether the state has finished or not
    pub fn process<T: Source>(&mut self, _env: &mut Env<T>) -> Result<bool> {
        todo!()
    }
}