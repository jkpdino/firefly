use firefly_span::Span;

use super::IntoDiagnostic;

pub enum StringError {
    InvalidHexSequence(String, Span),
    InvalidEscapeSequence(Span),
    NoHexSequence(Span),
}

impl IntoDiagnostic for StringError {
    fn into_diagnostic(&self) -> firefly_errors::diagnostic::Diagnostic {
        todo!()
    }
}