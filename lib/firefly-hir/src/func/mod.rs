//use signature::FuncSignature;

use crate::entity::{EntityKind, Id};

mod signature;

pub use signature::*;

/// Represents a function in the HIR.
#[derive(Debug, Clone)]
pub struct Func {
    pub id: Id<Func>,
    //signature: FuncSignature,
}

component!(base(EntityKind::Func) funcs: Func);

/// Represents an initializer in the HIR.
#[derive(Debug, Clone)]
pub struct Init {
    pub id: Id<Init>,
    //signature: FuncSignature,
}

component!(base(EntityKind::Init) inits: Init);
