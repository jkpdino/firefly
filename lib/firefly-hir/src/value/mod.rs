mod has_value;

use std::fmt::Debug;
use firefly_span::Span;
use crate::{
    entity::Id, func::Func, items::{Field, Global, StructDef}, stmt::Local, ty::Ty
};
pub use has_value::HasValue;

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Integer(String),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum ValueKind {
    Unit,
    Tuple(Vec<Value>),
    Literal(LiteralValue),

    FieldOf(Box<Value>, Id<Field>),

    StaticFunc(Id<Func>),
    InitFor(Id<StructDef>),
    BuiltinFunc(&'static str),

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