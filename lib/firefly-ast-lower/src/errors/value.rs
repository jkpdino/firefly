use firefly_ast::Name;
use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_hir::IntoDiagnostic;
use firefly_span::Span;

pub enum ValueError {
    BreakOutsideLoop(Span),
    UndefinedBreakLabel(Name),
    ContinueOutsideLoop(Span),
    UndefinedContinueLabel(Name),

    NotMutable(Span),
}

impl IntoDiagnostic for ValueError {
    fn into_diagnostic(&self, _context: &firefly_hir::HirContext) -> Diagnostic {
        match self {
            ValueError::BreakOutsideLoop(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Break outside of loop",))
                ).with_error_code(DiagnosticId::new("E0301"))
                 .with_source(*span)
            }
            ValueError::UndefinedBreakLabel(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Use of undefined label `{}` in break", name.item))
                ).with_error_code(DiagnosticId::new("E0302"))
                 .with_source(name.span)
            }
            ValueError::ContinueOutsideLoop(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Continue outside of loop",))
                ).with_error_code(DiagnosticId::new("E0303"))
                 .with_source(*span)
            }
            ValueError::UndefinedContinueLabel(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Use of undefined label `{}` in continue", name.item))
                ).with_error_code(DiagnosticId::new("E0304"))
                 .with_source(name.span)
            }
            ValueError::NotMutable(value) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Value is not mutable"))
                ).with_error_code(DiagnosticId::new("E0310"))
                 .with_source(*value)
            }
        }
    }
}