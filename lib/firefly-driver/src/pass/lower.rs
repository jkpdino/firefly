use firefly_ast::item::Item;

use crate::context::Context;

use super::ParallelPass;

pub struct LinkPass;

impl ParallelPass for LinkPass {
    type Input = Vec<Item>;
    type Output = Vec<Item>;

    fn process(&self, input: Self::Input, context: &mut Context) -> Self::Output {
        context.ast_lowerer.link_pass(&input);

        return input;
    }
}

pub struct LowerDefsPass;

impl ParallelPass for LowerDefsPass {
    type Input = Vec<Item>;
    type Output = Vec<Item>;

    fn process(&self, input: Self::Input, context: &mut Context) -> Self::Output {
        context.ast_lowerer.lower_item_defs(&input);

        return input;
    }
}

pub struct LowerCodePass;

impl ParallelPass for LowerCodePass {
    type Input = Vec<Item>;
    type Output = Vec<Item>;

    fn process(&self, input: Self::Input, context: &mut Context) -> Self::Output {
        context.ast_lowerer.lower_item_codes(&input);

        return input;
    }
}