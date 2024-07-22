//use signature::FuncSignature;

use crate::entity::{EntityKind, Id};

mod signature;

/// Represents a function in the HIR.
#[derive(Debug, Clone)]
pub struct Func {
    pub id: Id<Func>,
    //signature: FuncSignature,
}

component!(base(EntityKind::Func) funcs: Func);
