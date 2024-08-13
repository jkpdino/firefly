
use firefly_hir::value::{ElseValue, IfValue};
use firefly_interpret::ir::{code::{BasicBlockId, Terminator}, value::Immediate};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub(super) fn lower_if(&mut self, if_value: &IfValue, after_block: Option<BasicBlockId>) -> Immediate {
        let condition = self.lower_immediate(&if_value.condition);

        let then_block = self.vir.append_basic_block();
        let else_block = self.vir.append_basic_block();

        let after_block = after_block.unwrap_or_else(|| self.vir.append_basic_block());

        // Branch to the correct block
        self.vir.build_terminator(Terminator::branch_if(condition, then_block, else_block));

        // Lower the positive block
        self.vir.select_basic_block(then_block);
        self.lower_code_block(if_value.positive);
        self.vir.build_terminator(Terminator::branch(after_block));

        // Lower the negative block, if any
        self.vir.select_basic_block(else_block);
        match &if_value.negative {
            Some(ElseValue::Else(code_block)) => self.lower_code_block(*code_block),
            Some(ElseValue::ElseIf(if_value)) => { self.lower_if(if_value, Some(after_block)); },

            None => {}
        }
        self.vir.build_terminator(Terminator::branch(after_block));

        self.vir.select_basic_block(after_block);

        Immediate::void()
    }
}