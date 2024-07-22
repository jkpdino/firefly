use firefly_span::Span;

pub mod func;
pub mod items;

mod context;
mod entity;
mod generics;
mod path;
mod property;
mod stmt;
pub mod ty;
mod value;

pub use context::HirContext;
pub use entity::{Entity, Id};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Visibility {
    Public,
    Internal,
    FilePrivate,
    Private,
}

#[derive(Clone, Debug)]
pub struct Name {
    pub name: String,
    pub span: Span,
}
