use firefly_mangle::SymbolName;
use itertools::Itertools;

use crate::{ty::Ty, MirContext, util::{Container, DisplayInContext, IdFactory, UniqueId}};

use super::{bb::BasicBlock, BasicBlockId, Local};

pub struct Function {
    pub(crate) id:           UniqueId<Function>,
    pub(crate) name:         SymbolName,

    pub(crate) signature:    FunctionSignature,
    pub(crate) basic_blocks: Vec<BasicBlockId>,
    pub(crate) bb_factory:   IdFactory<BasicBlock>,

    pub(crate) locals:       Container<Local>,
}

pub struct FunctionSignature {
    pub(crate) parameters: Vec<Ty>,
    pub(crate) return_ty:  Ty
}

impl Function {
    pub fn locals(&self) -> &Vec<Local> {
        &self.locals
    }

    pub fn basic_blocks(&self) -> &Vec<BasicBlockId> {
        &self.basic_blocks
    }
}

impl DisplayInContext for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &MirContext) -> std::fmt::Result {
        let parameters = self.signature.parameters
            .iter()
            .enumerate()
            .map(|(i, param)| format!("%{i}: {}", context.display(param)))
            .format(", ");

        writeln!(f, "def {}({parameters}) -> {} {{", self.name, context.display(&self.signature.return_ty))?;

        for local in self.locals.iter() {
            writeln!(f, "    {}", context.display(local))?;
        }

        for &bb in &self.basic_blocks {
            let basic_block = context.get_basic_block(bb);

            writeln!(f, "{}", context.display(basic_block))?;
        }

        writeln!(f, "}}")
    }
}