use firefly_errors::emitter::Emitter;
use firefly_hir::{ty::{Ty, TyKind}, HirContext};

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
    pub fn can_assign_to(&self, sink: &Ty, source: &Ty) -> bool {
        match (&sink.kind, &source.kind) {
            (TyKind::Bool, TyKind::Bool) |
            (TyKind::Float, TyKind::Float) |
            (TyKind::Integer, TyKind::Integer) |
            (TyKind::String, TyKind::String) |
            (TyKind::Unit, TyKind::Unit) => true,

            (_, TyKind::Never) => true,

            (TyKind::StructDef(sink), TyKind::StructDef(source)) => sink == source,

            (
                TyKind::Func(sink_params, sink_return_ty),
                TyKind::Func(source_params, source_return_ty)
            ) => {
                if !self.can_assign_to(&sink_return_ty, &source_return_ty) {
                    return false;
                }

                if sink_params.len() != source_params.len() {
                    return false;
                }

                return sink_params.iter().zip(source_params)
                    .all(|(sink, source)| self.can_assign_to(sink, source))
            }

            (TyKind::Tuple(sink), TyKind::Tuple(source)) => {
                if sink.len() != source.len() {
                    return false;
                }

                return sink.iter().zip(source)
                    .all(|(sink, source)| self.can_assign_to(sink, source));
            }

            _ => false,
        }
    }

    pub fn throw(&self, error: TypeCheckError) {
        let diagnostic = error.into_diagnostic(self.context);

        self.emitter.emit(diagnostic).unwrap();
    }

    pub fn function_checker(&self, return_ty: &Ty) -> Typecheck {
        Typecheck { return_type: Some(return_ty.clone()), context: self }
    }
}