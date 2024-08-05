use firefly_span::Spanned;

use crate::{stmt::CodeBlock, Name, Path, PathSegment};

#[derive(Debug)]
pub enum Value {
    Tuple(Vec<Spanned<Value>>),
    IntegerLiteral(Name),
    StringLiteral(Name),
    Path(Path),
    Call(Box<Spanned<Value>>, Vec<Spanned<Value>>),
    Return(Option<Box<Spanned<Value>>>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    Break(Option<Name>),
    Continue(Option<Name>),
    Assign(Box<Spanned<Value>>, Box<Spanned<Value>>),
    Member(Box<Spanned<Value>>, PathSegment),
    Error,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Spanned<Value>,
    pub positive: CodeBlock,
    pub negative: Option<ElseStatement>
}

#[derive(Debug)]
pub enum ElseStatement {
    Else(CodeBlock),
    ElseIf(Box<IfStatement>)
}

#[derive(Debug)]
pub struct WhileStatement {
    pub label: Option<Name>,
    pub condition: Spanned<Value>,
    pub body: CodeBlock,
}