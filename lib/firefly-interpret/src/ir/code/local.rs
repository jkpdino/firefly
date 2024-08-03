use std::fmt::Display;

use firefly_span::Span;

use crate::{ir::{ty::Ty, value::{Place, PlaceKind}}, util::Id};

/// Declares a local value within the function. This
/// local doesn't have a value, but provides a place
/// for one to be stored.
pub struct Local {
    pub(crate) id: Id<Local>,
    pub(crate) ty: Ty,
}

impl Local {
    pub fn place_unspanned(&self) -> Place {
        Place { kind: Box::new(PlaceKind::Local(self.id)), ty: self.ty.clone(), span: Span::default() }
    }
}

impl Display for Id<Local> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.index())
    }
}


impl Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "local {}: {}", self.id, self.ty)
    }
}