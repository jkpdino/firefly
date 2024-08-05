use firefly_ast::Name;
use firefly_error_messages::DiagnosticMessage;
use firefly_errors::diagnostic::{Diagnostic, DiagnosticId, Level};
use firefly_hir::{value::Value, HirContext, IntoDiagnostic};
use firefly_span::Span;

pub enum SymbolError {
    NotFound(Name),
    NotVisible(Name, Span),
    NotFoundIn(Name, Span),

    NotAType(Span, Span),
    NotAValue(Span, Span),

    NoMembersOf(Value),
    NoMemberOn(Name, Value),
    MemberNotAValue(Name, Span),
}

impl IntoDiagnostic for SymbolError {
    fn into_diagnostic(&self, _context: &HirContext) -> firefly_errors::diagnostic::Diagnostic {
        match self {
            SymbolError::NotFound(name) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("symbol `{}` not found", name.item)))
                    .with_error_code(DiagnosticId::new("E0101"))
                    .with_source(name.span)
            }
            SymbolError::NotVisible(access, decl) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("symbol `{}` not visible", access.item)))
                    .with_error_code(DiagnosticId::new("E0102"))
                    .with_source(access.span)
                    .with_source(*decl)
            }
            SymbolError::NotFoundIn(access, decl) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("member `{}` not found", access.item)))
                    .with_error_code(DiagnosticId::new("E0103"))
                    .with_source(access.span)
                    .with_source(*decl)
            }
            SymbolError::NotAType(access, decl) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("symbol is not a type")))
                    .with_error_code(DiagnosticId::new("E0104"))
                    .with_source(*access)
                    .with_source(*decl)
            }
            SymbolError::NotAValue(access, decl) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("symbol is not a value")))
                    .with_error_code(DiagnosticId::new("E0105"))
                    .with_source(*access)
                    .with_source(*decl)
            }
            SymbolError::NoMembersOf(value) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("value of type `` has no members")))
                    .with_error_code(DiagnosticId::new("E0120"))
                    .with_source(value.span)
            }
            SymbolError::NoMemberOn(name, _) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("member `{}` not found for value of type ``", name.item)))
                    .with_error_code(DiagnosticId::new("E0121"))
                    .with_source(name.span)
            }
            SymbolError::MemberNotAValue(name, decl) => {
                Diagnostic::new(Level::Error, DiagnosticMessage::Str(format!("member `{}` is not a value", name.item)))
                    .with_error_code(DiagnosticId::new("E0122"))
                    .with_source(name.span)
                    .with_source(*decl)
            }
        }
    }
}