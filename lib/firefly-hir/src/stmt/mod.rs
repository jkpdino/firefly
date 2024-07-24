mod code_block;
mod binding;

pub use binding::Binding;

use firefly_span::Span;

use crate::{
    entity::{Entity, Id},
    ty::Ty,
    value::Value,
    Name,
};

pub enum StmtKind {
    Value(Value),
    Bind(Name, Ty, Value),
}

pub struct Stmt {
    //pub id: Id<Stmt>,
    pub kind: StmtKind,
    pub span: Span,
}