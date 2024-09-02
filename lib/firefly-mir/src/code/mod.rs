mod local;
mod bb;
mod func;
mod terminator;
mod global;

pub use local::*;
pub use bb::*;
pub use func::*;
pub use terminator::*;
pub use global::*;

use crate::{DisplayInContext, MirContext};

use super::value::{Immediate, Place};

pub enum InstructionKind {
    Assign(Place, Immediate),
    Eval(Immediate),
}

pub struct Instruction {
    pub kind: InstructionKind,
}

impl DisplayInContext for InstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        match self {
            InstructionKind::Assign(place, imm) => write!(f, "{place} := {}", context.display(imm)),
            InstructionKind::Eval(imm) => write!(f, "{}", context.display(imm))
        }
    }
}

impl DisplayInContext for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        self.kind.fmt(f, context)
    }
}