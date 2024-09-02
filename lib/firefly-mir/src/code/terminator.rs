use crate::{value::Immediate, MirContext, util::DisplayInContext};

use super::BasicBlockId;

pub enum TerminatorKind {
    Branch(BasicBlockId),
    BranchIf(Immediate, BasicBlockId, BasicBlockId),
    ReturnVoid,
    Return(Immediate)
}

/// A Terminator is a special instruction that ends a basic block.
/// Each basic block has exactly one terminator, which either moves
/// execution to another basic block, or exits the program.
pub struct Terminator {
    pub kind: TerminatorKind
}

impl Terminator {
    pub fn branch(bb: BasicBlockId) -> Self {
        Self {
            kind: TerminatorKind::Branch(bb)
        }
    }

    pub fn branch_if(condition: Immediate, positive: BasicBlockId, negative: BasicBlockId) -> Self {
        Self {
            kind: TerminatorKind::BranchIf(condition, positive, negative)
        }
    }

    pub fn returns(value: Immediate) -> Self {
        Self {
            kind: TerminatorKind::Return(value)
        }
    }

    pub fn returns_void() -> Self {
        Self {
            kind: TerminatorKind::ReturnVoid
        }
    }
}

impl DisplayInContext for TerminatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        match self {
            TerminatorKind::Branch(dest) => write!(f, "branch {}", dest.local_id),
            TerminatorKind::BranchIf(condition, positive, negative) => write!(f, "branch if {} ({} else {})", context.display(condition), positive.local_id, negative.local_id),
            TerminatorKind::ReturnVoid => write!(f, "return"),
            TerminatorKind::Return(value) => write!(f, "return {}", context.display(value))
        }
    }
}

impl DisplayInContext for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &MirContext) -> std::fmt::Result {
        self.kind.fmt(f, context)
    }
}