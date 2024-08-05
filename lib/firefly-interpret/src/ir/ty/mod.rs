pub mod struct_def;

use std::fmt::Formatter;

use itertools::Itertools;
use struct_def::StructDef;

use crate::util::{DisplayInContext, UniqueId};

use super::VirContext;

#[derive(Clone)]
pub enum TyKind {
    Integer,
    String,
    Bool,
    Float,

    Void,

    Struct(UniqueId<StructDef>),

    Tuple(Vec<Ty>),
    Func(Vec<Ty>, Ty),
}

#[derive(Clone)]
pub struct Ty {
    kind: Box<TyKind>,
}

impl Ty {
    pub fn new(kind: TyKind) -> Self {
        Self {
            kind: Box::new(kind)
        }
    }
}

impl DisplayInContext for TyKind {
    fn fmt(&self, f: &mut Formatter<'_>, context: &VirContext) -> std::fmt::Result {
        match self {
            TyKind::Integer => write!(f, "int"),
            TyKind::String => write!(f, "string"),
            TyKind::Float => write!(f, "float"),
            TyKind::Bool => write!(f, "bool"),
            
            TyKind::Void => write!(f, "void"),

            TyKind::Struct(_) => write!(f, "struct"),

            TyKind::Tuple(items) => write!(f, "({})", items.iter().map(|item| context.display(item)).format(", ")),
            TyKind::Func(params, return_ty) => write!(f, "func ({}) -> {}", params.iter().map(|item| context.display(item)).format(", "), context.display(return_ty))
        }
    }
}

impl DisplayInContext for Ty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, context: &VirContext) -> std::fmt::Result {
        self.kind.fmt(f, context)
    }
}