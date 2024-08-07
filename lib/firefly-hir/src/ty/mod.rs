use firefly_span::Span;

use crate::{entity::Id, items::StructDef, Entity, EntityKind};

mod has_type;

pub use has_type::HasType;

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
    Func(Vec<Ty>, Box<Ty>),

    Integer,
    String,
    Bool,
    Float,

    Never,
}

/// Represents a type in the HIR.
#[derive(Clone)]
pub struct Ty {
    pub id: Id<Ty>,
    pub kind: TyKind,
    pub span: Span,
}

impl Ty {
    pub fn new(kind: TyKind, span: Span) -> Ty {
        Ty {
            id: Default::default(),
            kind,
            span
        }
    }

    pub fn new_unspanned(kind: TyKind) -> Ty {
        Ty {
            id: Default::default(),
            kind,
            span: Default::default(),
        }
    }

    pub fn references(&self) -> Option<Id<Entity>> {
        match self.kind {
            TyKind::StructDef(id) => Some(id.as_base()),

            _ => None
        }
    }
}

component!(base(EntityKind::Ty) types: Ty);

impl std::fmt::Debug for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}