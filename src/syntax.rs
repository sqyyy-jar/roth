use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug)]
pub enum Type {
    Int,
    Float,
    String,
}

#[derive(Debug)]
pub struct ComposeType {
    pub types: Vec<Type>,
}

#[derive(Debug)]
pub enum TypeElement {
    Type { span: Span, value: Type },
    ComposeType { span: Span },
}

#[derive(Debug)]
pub enum Instruction {
    IntLiteral { span: Span, value: i64 },
    FloatLiteral { span: Span, value: f64 },
    StringLiteral { span: Span, value: Span },
    Call { span: Span },
}

#[derive(Debug)]
pub struct IfStatement {
    pub span: Span,
}

#[derive(Debug)]
pub struct WhileStatement {
    pub span: Span,
}

#[derive(Debug)]
pub enum CodeElement {
    Instruction(Instruction),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
}
