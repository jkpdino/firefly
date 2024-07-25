use crate::{ty::Ty, EntityKind, Id};

/// A binding represents a local variable binding
pub struct Local {
    pub id: Id<Local>,
    pub ty: Ty,
}

component!(base(EntityKind::Local) locals: Local);