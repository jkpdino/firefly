use std::fmt::Display;

use crate::{ir::value::{Immediate, Place}, util::{Id, UniqueId}};

use super::{Function, Instruction, InstructionKind, Terminator};

/// BasicBlocks 
#[derive(Copy, Clone)]
pub struct BasicBlockId {
    pub(crate) local_id:  Id<BasicBlock>,
    pub(crate) global_id: UniqueId<BasicBlock>,
    pub(crate) func_id:   UniqueId<Function>,
}

/// A basic block is a discrete block of code that runs together.
/// Each basic block must be terminated by a terminator that either
/// moves execution to another basic block or ends execution.
pub struct BasicBlock {
    id:           BasicBlockId,

    instructions: Vec<Instruction>,

    terminator:   Option<Terminator>
}

impl BasicBlock {
    pub fn new(id: BasicBlockId) -> Self {
        Self {
            id,
            instructions: Vec::new(),
            terminator: None,
        }
    }

    pub fn append_assign(&mut self, place: Place, imm: Immediate) {
        self.instructions.push(Instruction {
            kind: InstructionKind::Assign(place, imm)
        })
    }

    pub fn append_eval(&mut self, imm: Immediate) {
        self.instructions.push(Instruction {
            kind: InstructionKind::Eval(imm)
        })
    }

    pub fn append_terminator(&mut self, terminator: Terminator) {
        if self.terminator.is_some() {
            panic!();
        }

        self.terminator = Some(terminator);
    }
}

impl Display for Id<BasicBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@bb{}", self.index())
    }
}

impl Display for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}:", self.id.local_id)?;

        for instruction in &self.instructions {
            writeln!(f, "    {}", instruction)?;
        }

        Ok(())
    }
}