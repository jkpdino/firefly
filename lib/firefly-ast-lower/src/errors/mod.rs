mod module;
mod string;
mod symbol;

use firefly_errors::diagnostic::Diagnostic;

pub use module::*;
pub use string::*;
pub use symbol::*;

use crate::AstLowerer;

pub trait IntoDiagnostic {
    fn into_diagnostic(&self) -> Diagnostic;
}

impl AstLowerer {
    pub fn emit(&self, diagnostic: impl IntoDiagnostic) {
        let diagnostic = diagnostic.into_diagnostic();

        self.emitter.emit(diagnostic).unwrap();
    }
}