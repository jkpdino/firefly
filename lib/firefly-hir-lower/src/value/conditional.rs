
use std::collections::VecDeque;

use firefly_hir::value::{ElseValue, IfValue};
use firefly_mir::{code::Terminator, value::Immediate};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub(super) fn lower_if(&mut self, if_value: &IfValue) -> Immediate {
        let mut if_value_queue = VecDeque::new();
        if_value_queue.push_back(if_value);

        let mut blocks_to_link = Vec::new();

        while let Some(if_value) = if_value_queue.pop_front() {
            let condition = self.lower_immediate(&if_value.condition);

            let then_block = self.mir.append_basic_block();
            let else_block = self.mir.append_basic_block();

            // Branch to the correct block
            self.mir.build_terminator(Terminator::branch_if(condition, then_block, else_block));

            // Lower the positive block
            self.mir.select_basic_block(then_block);
            self.lower_code_block(if_value.positive);
            blocks_to_link.push(self.mir.current_basic_block_id());

            // Lower the negative block, if any
            self.mir.select_basic_block(else_block);
            match &if_value.negative {
                Some(ElseValue::Else(code_block)) => {
                    self.lower_code_block(*code_block);
                    blocks_to_link.push(self.mir.current_basic_block_id());
                },
                Some(ElseValue::ElseIf(if_value)) => { if_value_queue.push_back(if_value); },
                None => blocks_to_link.push(self.mir.current_basic_block_id()),
            }
        }

        let after_block = self.mir.append_basic_block();

        // Link all the blocks
        for block_id in blocks_to_link {
            self.mir.select_basic_block(block_id);

            if !self.mir.is_terminated() {
                self.mir.build_terminator(Terminator::branch(after_block));
            }
        }


        self.mir.select_basic_block(after_block);

        Immediate::void()
    }
}