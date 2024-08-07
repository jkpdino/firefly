use firefly_hir::{ty::TyKind, value::{LiteralValue, Value, ValueKind}};
use firefly_interpret::ir::{code::{Terminator, TerminatorKind}, ty::{Ty as VirTy, TyKind as VirTyKind}, value::{BinaryIntrinsic, BooleanBinaryOp, Comparison, ConstantValue, Immediate, ImmediateKind, IntegerBinaryOp, Place, PlaceKind, StringBinaryOp}};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_immediate(&mut self, value: &Value) -> Immediate {
        match &value.kind {
            ValueKind::Literal(LiteralValue::Integer(num)) => self.lower_integer(num, value.span),
            ValueKind::Literal(LiteralValue::String(string)) => self.lower_string(string, value.span),
            ValueKind::Literal(LiteralValue::Boolean(boolean)) => self.lower_bool(*boolean, value.span),

            ValueKind::Invoke(function, args) => self.lower_call(function, args),

            ValueKind::Return(value) => {
                let imm = self.lower_immediate(value);

                self.vir.build_terminator(Terminator {
                    kind: TerminatorKind::Return(imm)
                });

                Immediate::void()
            }

            _ => self.lower_place(value).move_out(),
        }
    }

    pub fn lower_place(&mut self, value: &Value) -> Place {
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

    pub fn lower_call(&mut self, func: &Value, args: &Vec<Value>) -> Immediate {
        let args = args.iter().map(|arg| self.lower_immediate(arg)).collect();

        let TyKind::Func(_, return_ty) = &func.ty.kind else {
            panic!();
        };

        let return_ty = self.lower_ty(&return_ty);

        match &func.kind {
            ValueKind::StaticFunc(static_func) => {
                let static_func = self.func_map[static_func];

                Immediate {
                    kind: Box::new(ImmediateKind::Call(static_func, args)),
                    ty: return_ty,
                    span: func.span,
                }
            }
            ValueKind::InstanceFunc(_, _) => todo!(),
            ValueKind::InitFor(_) => todo!(),
            ValueKind::BuiltinFunc(builtin_name) => self.lower_builtin(builtin_name, args, func.span),

            _ => unreachable!(),
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

    fn lower_bool(&self, boolean: bool, span: Span) -> Immediate {
        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::Bool(boolean))),
            ty: VirTy::new(VirTyKind::Bool),
            span,
        }
    }

    fn lower_builtin(&self, builtin_name: &str, args: Vec<Immediate>, span: Span) -> Immediate {
        // Check for binary
        let binary_kind = match builtin_name {
            "eq_int" => BinaryIntrinsic::Compare(Comparison::Equal),
            "neq_int" => BinaryIntrinsic::Compare(Comparison::NotEqual),
            "lt_int" => BinaryIntrinsic::Compare(Comparison::LessThan),
            "leq_int" => BinaryIntrinsic::Compare(Comparison::LessThanOrEqual),
            "gt_int" => BinaryIntrinsic::Compare(Comparison::GreaterThan),
            "geq_int" => BinaryIntrinsic::Compare(Comparison::GreaterThanOrEqual),
            "add" => BinaryIntrinsic::Integer(IntegerBinaryOp::Add),
            "sub" => BinaryIntrinsic::Integer(IntegerBinaryOp::Sub),
            "mul" => BinaryIntrinsic::Integer(IntegerBinaryOp::Mul),
            "div" => BinaryIntrinsic::Integer(IntegerBinaryOp::Div),
            "rem" => BinaryIntrinsic::Integer(IntegerBinaryOp::Rem),
            "left_shift" => BinaryIntrinsic::Integer(IntegerBinaryOp::ShiftLeft),
            "right_shift" => BinaryIntrinsic::Integer(IntegerBinaryOp::ShiftRight),
            "bitand" => BinaryIntrinsic::Integer(IntegerBinaryOp::BitAnd),
            "bitor" => BinaryIntrinsic::Integer(IntegerBinaryOp::BitOr),
            "bitxor" => BinaryIntrinsic::Integer(IntegerBinaryOp::BitXor),
            "and" => BinaryIntrinsic::Boolean(BooleanBinaryOp::And),
            "or" => BinaryIntrinsic::Boolean(BooleanBinaryOp::Or),
            "xor" => BinaryIntrinsic::Boolean(BooleanBinaryOp::Xor),
            "eq_bool" => BinaryIntrinsic::Compare(Comparison::Equal),
            "neq_bool" => BinaryIntrinsic::Compare(Comparison::NotEqual),
            "eq_str" => BinaryIntrinsic::Compare(Comparison::Equal),
            "neq_str" => BinaryIntrinsic::Compare(Comparison::NotEqual),
            "concat" => BinaryIntrinsic::String(StringBinaryOp::Concat),

            _ => return self.lower_unary_builtin(builtin_name, args, span),
        };

        let ty = match binary_kind {
            BinaryIntrinsic::Compare(_) => VirTy::new(VirTyKind::Bool),
            BinaryIntrinsic::Integer(_) => VirTy::new(VirTyKind::Integer),
            BinaryIntrinsic::Boolean(_) => VirTy::new(VirTyKind::Bool),
            BinaryIntrinsic::String(_) => VirTy::new(VirTyKind::String),
        };

        let [lhs, rhs] = &args[..] else {
            panic!()
        };

        Immediate {
            kind: Box::new(ImmediateKind::Binary(binary_kind, lhs.clone(), rhs.clone())),
            ty,
            span,
        }
    }

    #[allow(unused_variables)]
    fn lower_unary_builtin(&self, builtin_name: &str, args: Vec<Immediate>, span: Span) -> Immediate {
        todo!()
    }
}