use std::fmt::Display;

use itertools::Itertools;

pub enum TyKind {
    Integer,
    String,
    Bool,

    Void,

    Tuple(Vec<Ty>),
    Func(Vec<Ty>, Ty),
}
pub struct Ty {
    kind: Box<TyKind>,
}

impl Display for TyKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TyKind::Integer => write!(f, "int"),
            TyKind::String => write!(f, "string"),
            TyKind::Bool => write!(f, "bool"),
            
            TyKind::Void => write!(f, "void"),

            TyKind::Tuple(items) => write!(f, "({})", items.iter().format(", ")),
            TyKind::Func(params, return_ty) => write!(f, "func ({}) -> {}", params.iter().format(", "), return_ty)
        }
    }
}

impl Display for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}