use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};

use crate::{HirContext, Name};

use super::IntoDiagnostic;

pub enum ImportError {
    MultipleImports(Name, Name),
    NotVisible(Name),
    NotFound(Name),
}

impl IntoDiagnostic for ImportError {
    fn into_diagnostic(&self, _: &HirContext) -> Diagnostic {
        match self {
            ImportError::MultipleImports(original, import) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Multiple requests for importing `{}`", original.name))
                ).with_error_code(DiagnosticId::new("E0160"))
                 .with_source(original.span)
                 .with_source(import.span)
            }
            ImportError::NotVisible(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Item `{}` is not visible in the current context", name.name))
                ).with_error_code(DiagnosticId::new("E0161"))
                 .with_source(name.span)
            }
            ImportError::NotFound(name) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Item `{}` was not found", name.name))
                ).with_error_code(DiagnosticId::new("E0162"))
                 .with_source(name.span)
            }
        }
    }
}