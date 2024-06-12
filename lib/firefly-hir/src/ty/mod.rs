use crate::entity::Entity;

mod resolved;

/// Represents a type in the HIR.
/// 
/// This is a base, unresolved type that represents
/// how the type is used in the source code.
/// 
/// It does not contain any information about the actual type.
pub enum TypeKind {
    Unit,
    Tuple(Vec<Type>),
    Named(Path),
    Infer,
}

/// Represents a type in the HIR.
pub struct Type {
    id:   Id<Type>,
    kind: TypeKind,
    span: Span
}

impl Entity for Type {
    fn id(&self) -> Id<Self> {
        self.id
    }
}