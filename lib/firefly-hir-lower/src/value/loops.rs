use firefly_hir::value::WhileValue;
use firefly_interpret::ir::{code::{BasicBlockId, Terminator}, value::Immediate};

use crate::HirLowerer;

pub struct LoopMarker {
    pub start: BasicBlockId,
    pub end: BasicBlockId
}

impl HirLowerer<'_> {
    pub fn lower_while(&mut self, while_value: &WhileValue) -> Immediate {
        let while_start = self.vir.append_basic_block();
        let body = self.vir.append_basic_block();
        let while_end = self.vir.append_basic_block();

        // direct our current block to while_start
        self.vir.build_terminator(Terminator::branch(while_start));

        // build our selector
        self.vir.select_basic_block(while_start);
        let condition = self.lower_immediate(&while_value.condition);

        self.vir.build_terminator(Terminator::branch_if(condition, body, while_end));

        // track our loop for continue and breaks
        self.loop_map.insert(while_value.body, LoopMarker { start: while_start, end: while_end });

        // now lower the body
        self.vir.select_basic_block(body);
        self.lower_code_block(while_value.body);

        self.vir.build_terminator(Terminator::branch(while_start));

        // and finally, continue after the loop
        self.vir.select_basic_block(while_end);

        // Return void
        Immediate::void()
    }
}