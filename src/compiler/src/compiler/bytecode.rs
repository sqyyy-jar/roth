use std::collections::HashMap;

use roth_bytecode::{asm::Assembler, format::Binary};

use crate::{error::Result, state::function_state::FunctionState, syntax::Type};

/// Compilation task
pub struct CompileTask {
    pub binary: Binary,
    pub path: HashMap<String, Element>,
}

impl CompileTask {
    pub fn compile_function(&mut self, _state: &FunctionState) {
        todo!()
    }
}

impl Default for CompileTask {
    fn default() -> Self {
        Self {
            binary: Binary::default(),
            path: HashMap::with_capacity(0),
        }
    }
}

/// Importable extern collection of functions
///
/// Code is only required at runtime
pub trait Module {
    fn name(&self) -> String;

    fn load(&self, task: &mut CompileTask) -> Result<()>;

    fn shadable(&self) -> bool;

    fn shade(&self, binary: &mut Binary) -> Result<()>;
}

pub enum Element {
    Macro(Macro),
    Function(Function),
}

/// Compiletime macro
pub struct Macro {
    pub process: fn(
        task: &mut CompileTask,
        asm: &mut Assembler<Vec<u8>>,
        stack: &mut CompiletimeStack,
    ) -> Result<()>,
}

/// Compiletime function
pub struct Function {
    name: String,
    params: Vec<Type>,
    returns: Vec<Type>,
}

impl Function {
    pub fn new(name: String, params: Vec<Type>, returns: Vec<Type>) -> Self {
        Self {
            name,
            params,
            returns,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn signature(&self) -> (&[Type], &[Type]) {
        (&self.params, &self.returns)
    }
}

/// Exists for future splitting of `Type` and `StackType`
#[derive(Clone, Copy, PartialEq)]
pub enum StackType {
    Int,
    Float,
    String,
}

/// Enhanced stack that is able to check branching
pub struct CompiletimeStack {
    pub stack: Vec<StackType>,
    pub nested_stacks: Vec<NestedStack>,
}

impl CompiletimeStack {
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    pub fn push(&mut self, stack_type: StackType) {
        self.stack.push(stack_type);
        if !self.nested_stacks.is_empty() {
            self.nested_stacks.last_mut().unwrap().depth -= 1;
        }
    }

    pub fn pop(&mut self) -> Option<StackType> {
        if self.stack.is_empty() {
            return None;
        }
        if self.nested_stacks.is_empty() {
            self.stack.pop()
        } else {
            let res = self.stack.pop().unwrap();
            let last = self.nested_stacks.last_mut().unwrap();
            last.depth += 1;
            if last.depth > last.stack_bk.len() as isize {
                last.stack_bk.push(res);
            }
            Some(res)
        }
    }

    pub fn branch(&mut self) {
        self.nested_stacks.push(NestedStack {
            depth: 0,
            stack_bk: Vec::with_capacity(0),
        });
    }

    pub fn merge(&mut self) -> Result<()> {
        if self.nested_stacks.is_empty() {
            todo!("Nothing to merge")
        }
        let stack = self.nested_stacks.pop().unwrap();
        let stack_len = stack.stack_bk.len();
        let root_stack_len = self.stack.len();
        if stack_len > root_stack_len {
            todo!("Stack overflow")
        }
        for (i, element) in stack.stack_bk.into_iter().enumerate() {
            if self.stack[root_stack_len - 1 - i] != element {
                todo!("Invalid stack to merge")
            }
        }
        Ok(())
    }
}

impl Default for CompiletimeStack {
    fn default() -> Self {
        Self {
            stack: Vec::with_capacity(0),
            nested_stacks: Vec::with_capacity(0),
        }
    }
}

pub struct NestedStack {
    depth: isize,
    stack_bk: Vec<StackType>,
}
