use firefly_ast::Name;
use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_hir::{HirContext, IntoDiagnostic};

pub enum DeclarationError {
    GlobalVarNoDefault(Name),
}

impl IntoDiagnostic for DeclarationError {
    fn into_diagnostic(&self, _: &HirContext) -> Diagnostic {
        match self {
            DeclarationError::GlobalVarNoDefault(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Global variable `{}` has no default", name.item))
                ).with_error_code(DiagnosticId::new("E0601"))
                 .with_source(name.span)
            }
        }
    }
}