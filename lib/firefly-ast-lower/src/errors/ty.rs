use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_hir::IntoDiagnostic;
use firefly_span::Span;

pub enum TypeError {
    CantCall(Span)
}

impl IntoDiagnostic for TypeError {
    fn into_diagnostic(&self, _: &firefly_hir::HirContext) -> Diagnostic {
        match self {
            Self::CantCall(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Value can't be called",))
                ).with_error_code(DiagnosticId::new("E0501"))
                 .with_highlight(*span)
            }
        }
    }
}