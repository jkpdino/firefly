mod has_value;

use std::fmt::Debug;
use firefly_span::Span;
use crate::{
    entity::Id, func::Func, items::{Field, Global, StructDef}, stmt::{CodeBlock, Local}, ty::Ty, Name
};
pub use has_value::*;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub struct IfValue {
    pub condition: Value,
    pub positive: Id<CodeBlock>,
    pub negative: Option<ElseValue>
}

#[derive(Debug, Clone)]
pub enum ElseValue {
    Else(Id<CodeBlock>),
    ElseIf(Box<IfValue>),
}

#[derive(Debug, Clone)]
pub struct WhileValue {
    pub label:     Option<Name>,
    pub condition: Value,
    pub body:      Id<CodeBlock>,
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    Unit,
    Tuple(Vec<Value>),
    Literal(LiteralValue),

    FieldOf(Box<Value>, Id<Field>),

    StaticFunc(Id<Func>),
    InstanceFunc(Box<Value>, Id<Func>),
    InitFor(Id<StructDef>),
    BuiltinFunc(&'static str),

    Return(Box<Value>),
    Break(Id<CodeBlock>),
    Continue(Id<CodeBlock>),

    If(Box<IfValue>),
    While(Box<WhileValue>),

    Invoke(Box<Value>, Vec<Value>),
    Local(Id<Local>),
    Global(Id<Global>),
}

#[derive(Clone)]
pub struct Value {
    //id: Id<Value>,
    pub kind: ValueKind,
    pub ty: Ty,
    pub span: Span,
}

impl Value {
    pub fn new(kind: ValueKind, ty: Ty, span: Span) -> Value {
        Value {
            kind,
            ty,
            span
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}