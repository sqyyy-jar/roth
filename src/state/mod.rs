use crate::{error::Result, util::source::Source};

use self::{
    function_state::FunctionState, if_state::IfState, root_state::RootState,
    string_state::StringState, while_state::WhileState,
};

pub mod function_state;
pub mod if_state;
pub mod root_state;
pub mod string_state;
pub mod while_state;

/// Holds:
/// * the stack of all states
/// * the the last popped state to be consumed by the top state
pub struct Env<T: Source> {
    source: T,
    tmp_stack: Vec<State>,
    result: Option<State>,
}

impl<T: Source> Env<T> {}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    New,
    Active,
    Waiting,
}

impl Status {
    pub fn is_new(self) -> bool {
        self == Self::New
    }

    pub fn is_active(self) -> bool {
        self == Self::Active
    }

    pub fn is_waiting(self) -> bool {
        self == Self::Waiting
    }
}

#[derive(Debug)]
pub enum State {
    Root(RootState),
    Function(FunctionState),
    If(IfState),
    While(WhileState),
    String(StringState),
}

impl State {
    /// Returns wether the state has finished or not
    fn process<T: Source>(&mut self, env: &mut Env<T>) -> Result<bool> {
        match self {
            State::Root(it) => it.process(env),
            State::Function(it) => it.process(env),
            State::If(it) => it.process(env),
            State::While(it) => it.process(env),
            State::String(it) => it.process(env),
        }
    }
}

pub fn process(initial_state: State, source: impl Source) -> Result<State> {
    let mut states = Vec::new();
    let mut env = Env {
        source,
        tmp_stack: Vec::with_capacity(1),
        result: None,
    };
    states.push(initial_state);
    loop {
        if !states.last_mut().unwrap().process(&mut env)? {
            states.append(&mut env.tmp_stack);
            continue;
        }
        states.append(&mut env.tmp_stack);
        if states.len() == 1 {
            return Ok(states.pop().unwrap());
        }
        env.result = Some(states.pop().unwrap());
    }
}
