use std::fmt::Display;

use firefly_span::Span;

use crate::{ir::{code::Local, ty::Ty}, util::Id};

use super::{Immediate, ImmediateKind};

pub enum PlaceKind {
    /// A value local to a function
    Local(Id<Local>),
}

pub struct Place {
    pub kind: Box<PlaceKind>,
    pub ty:   Ty,
    pub span: Span,
}

impl Place {
    pub fn move_out(self) -> Immediate {
        let ty = self.ty.clone();
        let span = self.span;
        let kind = ImmediateKind::Move(self);

        return Immediate { kind: Box::new(kind), ty, span };
    }
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