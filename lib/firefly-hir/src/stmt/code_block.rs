use firefly_span::Span;

use super::Stmt;

pub struct CodeBlock {
    pub stmts: Vec<Stmt>,
    pub span:  Span,
}