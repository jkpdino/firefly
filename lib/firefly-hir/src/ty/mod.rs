use firefly_span::Span;

use crate::{
    entity::{Entity, Id},
    items::StructDef,
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
    StructDef(Id<StructDef>),

    Integer,
    String,
    Bool,
    Float,
}

/// Represents a type in the HIR.
#[derive(Debug, Clone)]
pub struct Ty {
    pub kind: TyKind,
    pub span: Span,
}
