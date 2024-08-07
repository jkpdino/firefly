use firefly_hir_lower::lower;

use crate::context::Context;

use super::Pass;

pub struct LowerHirPass;

impl Pass for LowerHirPass {
    type Input = ();
    type Output = ();

    fn process(&self, _: Self::Input, context: &mut Context) -> Self::Output {
        lower(context.ast_lowerer.context(), context.vir_context);

        println!("{}", context.vir_context);

        return ();
    }
}