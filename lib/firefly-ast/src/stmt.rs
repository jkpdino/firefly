use firefly_span::Spanned;

use crate::{ty::Ty, value::Value, Name};

#[derive(Debug)]
pub enum Stmt {
    Value(Spanned<Value>),
    Bind(Name, Spanned<Ty>, Spanned<Value>),
    Error
}

#[derive(Debug)]
pub struct CodeBlock {
    pub stmts: Vec<Spanned<Stmt>>,
}

impl CodeBlock {
    pub fn new(stmts: Vec<Spanned<Stmt>>) -> Self {
        Self {
            stmts
        }
    }
}