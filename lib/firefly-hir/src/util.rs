use crate::entity::{EntityKind, Id};
use firefly_span::Span;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Visibility {
    Public,
    Internal,
    FilePrivate,
    Private,
    Local,
}

#[derive(Clone)]
pub struct Name {
    pub name: String,
    pub span: Span,
}

#[derive(Default, Debug)]
pub struct Root {
    id: Id<Root>,
}

component!(base(EntityKind::Root) roots: Root);


impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.name)
    }
}