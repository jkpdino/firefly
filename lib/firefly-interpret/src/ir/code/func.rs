use crate::{ir::ty::Ty, util::{Container, IdFactory, UniqueId}};

use super::{bb::BasicBlock, BasicBlockId, Local};

pub struct Function {
    pub(crate) id:           UniqueId<Function>,
    pub(crate) name:         String,

    pub(crate) signature:    FunctionSignature,
    pub(crate) basic_blocks: Vec<BasicBlockId>,
    pub(crate) bb_factory:   IdFactory<BasicBlock>,

    pub(crate) locals:       Container<Local>,
}

pub struct FunctionSignature {
    pub(crate) parameters: Vec<Ty>,
    pub(crate) return_ty:  Ty
}