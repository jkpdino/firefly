use firefly_span::Span;

use crate::{
    entity::{Entity, Id},
    path::Path,
};

/// Represents a type in the HIR.
///
/// This is a base, unresolved type that represents
/// how the type is used in the source code.
///
/// It does not contain any information about the actual type.
#[derive(Debug, Clone)]
pub enum TyKind {
    Unit,
    Tuple(Vec<Ty>),
    Named(Path),
    Infer,
}

/// Represents a type in the HIR.
#[derive(Debug, Clone)]
pub struct Ty {
    id: Id<Ty>,
    kind: TyKind,
    span: Span,
}

impl Entity for Ty {
    fn id(&self) -> Id<Self> {
        self.id
    }
}
