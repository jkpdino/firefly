use firefly_hir::{stmt::CodeBlock as HirCodeBlock, Id};
use firefly_span::Spanned;

use crate::{ty::Ty, value::Value, Name};

#[derive(Debug, Clone)]
pub enum Stmt {
    Value(Spanned<Value>, bool),
    Bind(Name, Spanned<Ty>, Spanned<Value>),
    Semicolon,
    Error,
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub id: Id<HirCodeBlock>,
    pub stmts: Vec<Spanned<Stmt>>,
    pub yields: Option<Spanned<Value>>,
}

impl CodeBlock {
    pub fn new(stmts: Vec<Spanned<Stmt>>, yields: Option<Spanned<Value>>) -> Self {
        Self {
            id: Default::default(),
            stmts,
            yields,
        }
    }
}
