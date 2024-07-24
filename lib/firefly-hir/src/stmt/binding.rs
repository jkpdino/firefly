use crate::{ty::Ty, EntityKind, Id};

/// A binding represents a local variable binding
pub struct Binding {
    pub id: Id<Binding>,
    pub ty: Ty,
}

component!(base(EntityKind::Binding) bindings: Binding);