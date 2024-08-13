use firefly_hir::{stmt::CodeBlock, value::WhileValue, Id};
use firefly_mir::{code::{BasicBlockId, Terminator}, value::Immediate};

use crate::HirLowerer;

pub struct LoopMarker {
    pub start: BasicBlockId,
    pub end: BasicBlockId
}

impl HirLowerer<'_> {
    pub(super) fn lower_while(&mut self, while_value: &WhileValue) -> Immediate {
        let while_start = self.mir.append_basic_block();
        let body = self.mir.append_basic_block();
        let while_end = self.mir.append_basic_block();

        // direct our current block to while_start
        self.mir.build_terminator(Terminator::branch(while_start));

        // build our selector
        self.mir.select_basic_block(while_start);
        let condition = self.lower_immediate(&while_value.condition);

        self.mir.build_terminator(Terminator::branch_if(condition, body, while_end));

        // track our loop for continue and breaks
        self.loop_map.insert(while_value.body, LoopMarker { start: while_start, end: while_end });

        // now lower the body
        self.mir.select_basic_block(body);
        self.lower_code_block(while_value.body);

        self.mir.build_terminator(Terminator::branch(while_start));

        // and finally, continue after the loop
        self.mir.select_basic_block(while_end);

        // Return void
        Immediate::void()
    }

    pub(super) fn lower_break(&mut self, code_block: Id<CodeBlock>) -> Immediate {
        let Some(LoopMarker { end, .. }) = self.loop_map.get(&code_block) else {
            panic!("internal compiler error: expected code block to be tracked");
        };

        self.mir.build_terminator(Terminator::branch(*end));

        Immediate::void()
    }

    pub(super) fn lower_continue(&mut self, code_block: Id<CodeBlock>) -> Immediate {
        let Some(LoopMarker { start, .. }) = self.loop_map.get(&code_block) else {
            panic!("internal compiler error: expected code block to be tracked");
        };

        self.mir.build_terminator(Terminator::branch(*start));

        Immediate::void()
    }
}