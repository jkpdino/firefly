use firefly_errors::emitter::Emitter;
use firefly_hir::{ty::Ty, value::Value, HirContext};

use crate::{errors::TypeCheckError, Typecheck};

pub struct TypecheckContext<'a> {
    context: &'a HirContext,
    emitter: &'a Emitter,
}

impl<'a> TypecheckContext<'a> {
    pub fn new(context: &'a HirContext, emitter: &'a Emitter) -> Self {
        Self {
            context,
            emitter
        }
    }
    pub fn can_assign_to(&self, ty: &Ty, value: &Value) -> bool {
        false
    }

    pub fn throw(&self, error: TypeCheckError) {
        let diagnostic = error.into_diagnostic(self.context);

        self.emitter.emit(diagnostic).unwrap();
    }

    pub fn function_checker(&self, return_ty: &Ty) -> Typecheck {
        Typecheck { return_type: Some(return_ty.clone()), context: self }
    }
}