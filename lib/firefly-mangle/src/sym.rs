use std::fmt::Display;

use crate::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolName {
    Func(Path),
    Struct(Path),
    Var(Path),

    Custom(String)
}

impl Display for SymbolName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolName::Func(path) => write!(f, "_F{}", path),
            SymbolName::Struct(path) => write!(f, "_S{}", path),
            SymbolName::Var(path) => write!(f, "_V{}", path),
            SymbolName::Custom(name) => write!(f, "{}", name),
        }
    }
}