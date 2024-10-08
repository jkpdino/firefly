use std::fmt::Display;

use crate::{value::{Immediate, Place}, MirContext, util::{DisplayInContext, Id, UniqueId}};

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
    pub(crate) id:           BasicBlockId,

    pub(crate) instructions: Vec<Instruction>,

    pub(crate) terminator:   Option<Terminator>
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
            return;
            //panic!("{:?}", self.terminator);
        }

        self.terminator = Some(terminator);
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }

    pub fn terminator(&self) -> Option<&Terminator> {
        self.terminator.as_ref()
    }
}

impl Display for Id<BasicBlock> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bb{}", self.index())
    }
}

impl DisplayInContext for BasicBlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        writeln!(f, "{}:", self.id.local_id)?;

        for instruction in &self.instructions {
            writeln!(f, "    {}", context.display(instruction))?;
        }

        if let Some(terminator) = &self.terminator {
            writeln!(f, "    {}", context.display(terminator))?;
        }

        Ok(())
    }
}