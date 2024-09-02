use crate::context::Context;

use super::Pass;

pub struct TyCheckPass;

impl Pass for TyCheckPass {
    type Input = ();
    type Output = ();

    fn process(&self, _: Self::Input, context: &mut Context) -> Self::Output {
        firefly_ty_check::pass::type_check_context(context.ast_lowerer.context_mut(), &context.emitter);

        return ();
    }
}