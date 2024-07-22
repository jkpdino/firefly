use signature::FuncSignature;

use crate::entity::{Entity, Id};

mod signature;

/// Represents a function in the HIR.
#[derive(Debug, Clone)]
pub struct Func {
    id: Id<Func>,
    signature: FuncSignature,
}

impl Entity for Func {
    fn id(&self) -> Id<Self> {
        self.id
    }
}