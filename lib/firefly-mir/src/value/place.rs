use std::fmt::Display;

use firefly_span::Span;

use crate::{code::{Global, Local}, ty::Ty, util::{DisplayInContext, Id}};

use super::{Immediate, ImmediateKind};

#[derive(Clone)]
pub enum PlaceKind {
    /// A value local to a function
    Local(Id<Local>),

    /// A value accessible throughout the program
    Global(Id<Global>),

    /// A field of a struct or a tuple
    Field(Place, usize),
}

#[derive(Clone)]
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
            PlaceKind::Local(local_id) => write!(f, "{local_id}"),
            PlaceKind::Global(global_id) => write!(f, "{global_id}"),
            PlaceKind::Field(place, index) => write!(f, "{place}.{index}"),
        }
    }
}

impl DisplayInContext for PlaceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &crate::MirContext) -> std::fmt::Result {
        match self {
            PlaceKind::Local(local_id) => write!(f, "{local_id}"),
            PlaceKind::Global(global_id) => {
                let global = context.get_global(*global_id);

                write!(f, "@{}", global.name)
            }
            PlaceKind::Field(place, index) => write!(f, "{place}.{index}"),
        }
    }
}

impl Display for Place {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}