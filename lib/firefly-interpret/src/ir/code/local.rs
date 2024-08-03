use std::fmt::Display;

use crate::ir::ty::Ty;

/// Refers to a specific indexed local in the current function
pub struct LocalId(pub(crate) usize);

/// Declares a local value within the function. This
/// local doesn't have a value, but provides a place
/// for one to be stored.
pub struct Local {
    pub(crate) id: LocalId,
    pub(crate) ty: Ty,
}

impl Display for LocalId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "%{}", self.0)
    }
}


impl Display for Local {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "local {}: {}", self.id, self.ty)
    }
}