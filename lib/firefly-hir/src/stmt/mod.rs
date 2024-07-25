mod code_block;
mod binding;

use std::fmt::Debug;

pub use binding::Binding;
pub use code_block::CodeBlock;

use firefly_span::Span;

use crate::{
    ty::Ty,
    value::Value,
    Name,
};

#[derive(Debug, Clone)]
pub enum StmtKind {
    Value(Value),
    Bind(Name, Ty, Value),
}

#[derive(Clone)]
pub struct Stmt {
    //pub id: Id<Stmt>,
    pub kind: StmtKind,
    pub span: Span,
}

impl Stmt {
    pub fn new(kind: StmtKind, span: Span) -> Stmt {
        Stmt {
            kind,
            span
        }
    }
}

impl Debug for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}