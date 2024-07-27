use firefly_errors::diagnostic::Diagnostic;

use crate::HirContext;

pub trait IntoDiagnostic {
    fn into_diagnostic(&self, context: &HirContext) -> Diagnostic;
}

impl HirContext {
    pub fn emit(&self, diagnostic: impl IntoDiagnostic) {
        let diagnostic = diagnostic.into_diagnostic(self);

        self.emitter.emit(diagnostic).unwrap();
    }
}