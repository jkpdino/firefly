
use firefly_interpret::ir::{ty::{Ty as VirTy, TyKind as VirTyKind}, value::{ConstantValue, Immediate, ImmediateKind}};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub(super) fn lower_integer(&self, value: &str, span: Span) -> Immediate {
        let value = if value.starts_with("0b") {
            u64::from_str_radix(&value[2..], 2).unwrap_or(u64::MAX)
        } else if value.starts_with("0x") {
            u64::from_str_radix(&value[2..], 16).unwrap_or(u64::MAX)
        } else if value.starts_with("0o") {
            u64::from_str_radix(&value[2..], 8).unwrap_or(u64::MAX)
        } else {
            u64::from_str_radix(value, 10).unwrap_or(u64::MAX)
        };

        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::Integer(value))),
            ty: VirTy::new(VirTyKind::Integer),
            span,
        }
    }

    pub(super) fn lower_string(&self, value: &str, span: Span) -> Immediate {
        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::String(value.to_string()))),
            ty: VirTy::new(VirTyKind::String),
            span,
        }
    }

    pub(super) fn lower_bool(&self, boolean: bool, span: Span) -> Immediate {
        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::Bool(boolean))),
            ty: VirTy::new(VirTyKind::Bool),
            span,
        }
    }

    pub(super) fn lower_float(&self, float: &str, span: Span) -> Immediate {
        let float = float.parse().unwrap_or(f64::NAN);

        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::Float(float))),
            ty: VirTy::new(VirTyKind::Float),
            span,
        }
    }
}