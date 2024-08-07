use std::u64;

use firefly_hir::{ty::{Ty as HirTy, TyKind as HirTyKind}, value::{LiteralValue, Value, ValueKind}};
use firefly_interpret::ir::{ty::{Ty as VirTy, TyKind as VirTyKind}, value::{ConstantValue, Immediate, ImmediateKind, Place, PlaceKind}};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_immediate(&self, value: &Value) -> Immediate {
        match &value.kind {
            ValueKind::Literal(LiteralValue::Integer(num)) => self.lower_integer(num, value.span),
            ValueKind::Literal(LiteralValue::String(string)) => self.lower_string(string, value.span),

            _ => self.lower_place(value).move_out(),
        }
    }

    pub fn lower_place(&self, value: &Value) -> Place {
        match &value.kind {
            ValueKind::Local(id) => {
                let vir_local = self.local_map[id];
                let local = self.hir.get(*id);

                Place {
                    kind: Box::new(PlaceKind::Local(vir_local)),
                    ty: self.lower_ty(&local.ty),
                    span: value.span,
                }
            }

            _ => todo!(),
        }
    }

    fn lower_integer(&self, value: &str, span: Span) -> Immediate {
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

    fn lower_string(&self, value: &str, span: Span) -> Immediate {
        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::String(value.to_string()))),
            ty: VirTy::new(VirTyKind::String),
            span,
        }
    }
}