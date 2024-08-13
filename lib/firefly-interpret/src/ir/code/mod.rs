mod local;
mod bb;
mod func;
mod terminator;
mod global;

use std::fmt::Display;

pub use local::*;
pub use bb::*;
pub use func::*;
pub use terminator::*;
pub use global::*;

use super::value::{Immediate, Place};

pub enum InstructionKind {
    Assign(Place, Immediate),
    Eval(Immediate),
}

pub struct Instruction {
    pub kind: InstructionKind,
}

impl Display for InstructionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionKind::Assign(place, imm) => write!(f, "{place} := {imm}"),
            InstructionKind::Eval(imm) => write!(f, "{imm}")
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}