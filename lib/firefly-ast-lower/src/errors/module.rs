use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_hir::Name;
use firefly_span::Span;

use super::IntoDiagnostic;

pub enum ModuleError {
    NotAModule(Name),
    ModuleDeclarationInside(Span),
    MultipleModulesFound(Vec<Span>),
    NoModuleFound
}

impl IntoDiagnostic for ModuleError {
    fn into_diagnostic(&self) -> firefly_errors::diagnostic::Diagnostic {
        match self {
            ModuleError::NotAModule(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Item {} is not a module", name.name))
                ).with_error_code(DiagnosticId::new("E0106"))
                 .with_source(name.span)
            }
            ModuleError::ModuleDeclarationInside(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Module declarations must appear directly within a file"))
                ).with_error_code(DiagnosticId::new("E0150"))
                 .with_source(*span)
            }
            ModuleError::MultipleModulesFound(spans) => {
                let diag = Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Only one module declaration can appear per file"))
                ).with_error_code(DiagnosticId::new("E0151"));

                return spans.iter().fold(diag, |diag, span| diag.with_source(*span));
            }
            ModuleError::NoModuleFound => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Every file must contain a module declaration"))
                ).with_error_code(DiagnosticId::new("E0152"))
            }
        }
    }
}