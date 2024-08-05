use firefly_hir_lower::lower;
use firefly_interpret::ir::VirContext;

use crate::context::Context;

use super::Pass;

pub struct LowerHirPass;

impl Pass for LowerHirPass {
    type Input = ();
    type Output = VirContext;

    fn process(&self, _: Self::Input, context: &mut Context) -> Self::Output {
        let vir_context = lower(context.ast_lowerer.context());

        println!("{}", vir_context);

        return vir_context;
    }
}