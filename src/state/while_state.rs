use crate::{error::Result, util::source::Source};

use super::{loop_state::LoopState, Env};

/// Holds:
/// * previous stack size
/// * `Instruction`s or `State`s (if, while) in the while element
///
/// Can be incomplete at any time
#[derive(Debug)]
pub struct WhileState {
    pub(super) inner: LoopState,
}

impl WhileState {
    pub fn with_start_index(start_index: usize) -> Self {
        Self {
            inner: LoopState::with_start_index(start_index),
        }
    }

    /// Returns wether the state has finished or not
    pub fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        self.inner.process(env)
    }
}
