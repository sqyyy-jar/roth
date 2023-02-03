use std::collections::HashMap;

pub struct Path {
    pub elements: HashMap<String, Element>,
}

pub enum Element {
    Macro(Macro),
    Function(Function),
}

pub struct Macro {}

pub struct Function {}
