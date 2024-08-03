use std::fmt::Display;

use firefly_span::Span;

use crate::ir::{code::LocalId, ty::Ty};

pub enum PlaceKind {
    /// A value local to a function
    Local(LocalId),
}

pub struct Place {
    kind: Box<PlaceKind>,
    ty:   Ty,
    span: Span,
}

impl Display for PlaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlaceKind::Local(local_id) => write!(f, "{local_id}")
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}