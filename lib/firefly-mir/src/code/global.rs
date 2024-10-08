use std::fmt::Display;

use firefly_mangle::SymbolName;
use firefly_span::Span;

use crate::{ty::Ty, value::{Place, PlaceKind}, MirContext, util::{DisplayInContext, Id, UniqueId}};

/// A Global value is accessible from anywhere in the module.
/// It is initialized at the start of the program and can be
/// mutated at any time.
pub struct Global {
    pub(crate) id:   UniqueId<Global>,
    pub(crate) name: SymbolName,
    pub(crate) ty:   Ty,
}

impl Global {
    pub fn id(&self) -> UniqueId<Global> {
        self.id
    }

    pub fn place_unspanned(&self) -> Place {
        Place { kind: Box::new(PlaceKind::Global(self.id)), ty: self.ty.clone(), span: Span::default() }
    }
}

impl Display for Id<Global> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.index())
    }
}


impl DisplayInContext for Global {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        write!(f, "global {}: {}", self.name, context.display(&self.ty))
    }
}