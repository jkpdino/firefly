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
        todo!()
    }
}