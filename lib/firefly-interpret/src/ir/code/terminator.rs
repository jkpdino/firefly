use std::fmt::Display;

use crate::{ir::{value::Immediate, VirContext}, util::DisplayInContext};

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

impl DisplayInContext for TerminatorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &VirContext) -> std::fmt::Result {
        match self {
            TerminatorKind::Branch(dest) => write!(f, "branch {}", dest.local_id),
            TerminatorKind::BranchIf(condition, positive, negative) => write!(f, "branch if {condition} ({} else {})", positive.local_id, negative.local_id),
            TerminatorKind::ReturnVoid => write!(f, "return"),
            TerminatorKind::Return(_) => todo!(),
        }
    }
}

impl DisplayInContext for Terminator {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &VirContext) -> std::fmt::Result {
        self.kind.fmt(f, context)
    }
}