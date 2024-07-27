use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_span::Span;

use super::IntoDiagnostic;

pub enum StringError {
    InvalidHexSequence(String, Span),
    InvalidEscapeSequence(Span),
    NoHexSequence(Span),
}

impl IntoDiagnostic for StringError {
    fn into_diagnostic(&self) -> firefly_errors::diagnostic::Diagnostic {
        match self {
            StringError::InvalidEscapeSequence(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Invalid escape sequence"))
                ).with_error_code(DiagnosticId::new("E0201"))
                 .with_source(*span)
            }
            StringError::NoHexSequence(span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Hex code escape sequence has no hex code"))
                ).with_error_code(DiagnosticId::new("E0202"))
                .with_source(*span)
            }
            StringError::InvalidHexSequence(seq, span) => {
                Diagnostic::new(Level::Error,
                    DiagnosticMessage::Str(format!("Hex code {seq} is not a valid character"))
                ).with_error_code(DiagnosticId::new("E0203"))
                 .with_source(*span)
            }
        }
    }
}