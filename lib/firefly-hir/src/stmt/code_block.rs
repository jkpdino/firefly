use std::fmt::Debug;

use firefly_span::Span;

use crate::{value::Value, EntityKind, Id};

use super::Stmt;

#[derive(Clone)]
pub struct CodeBlock {
    pub id:     Id<CodeBlock>,
    pub stmts:  Vec<Stmt>,
    pub yields: Option<Value>,
    pub span:   Span,
}

component!(base(EntityKind::CodeBlock) code_blocks: CodeBlock);

impl Debug for CodeBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CodeBlock {{")?;

        for stmt in &self.stmts {
            writeln!(f, "  {stmt:?}")?;
        }

        if let Some(yields) = &self.yields  {
            writeln!(f, "  {yields:?}")?;
        }

        write!(f, "}}")
    }
}