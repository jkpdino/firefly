use crate::entity::{EntityKind, Id};
use firefly_span::Span;

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

#[derive(Default)]
pub struct Root {
    id: Id<Root>,
}

component!(base(EntityKind::Root) roots: Root);
