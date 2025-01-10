use firefly_mir::{
    ty::{Ty as MirTy, TyKind as MirTyKind},
    value::{
        BinaryIntrinsic, BooleanBinaryOp, Comparison, FloatBinaryOp, Immediate, ImmediateKind,
        IntegerBinaryOp, StringBinaryOp, UnaryIntrinsic,
    },
};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub(super) fn lower_builtin(
        &self,
        builtin_name: &str,
        args: Vec<Immediate>,
        span: Span,
    ) -> Immediate {
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

            "eq_float" => BinaryIntrinsic::Compare(Comparison::Equal),
            "neq_float" => BinaryIntrinsic::Compare(Comparison::NotEqual),
            "lt_float" => BinaryIntrinsic::Compare(Comparison::LessThan),
            "leq_float" => BinaryIntrinsic::Compare(Comparison::LessThanOrEqual),
            "gt_float" => BinaryIntrinsic::Compare(Comparison::GreaterThan),
            "geq_float" => BinaryIntrinsic::Compare(Comparison::GreaterThanOrEqual),

            "fadd" => BinaryIntrinsic::Float(FloatBinaryOp::Add),
            "fsub" => BinaryIntrinsic::Float(FloatBinaryOp::Sub),
            "fmul" => BinaryIntrinsic::Float(FloatBinaryOp::Mul),
            "fdiv" => BinaryIntrinsic::Float(FloatBinaryOp::Div),
            "frem" => BinaryIntrinsic::Float(FloatBinaryOp::Rem),
            "fpow" => BinaryIntrinsic::Float(FloatBinaryOp::Pow),

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
            BinaryIntrinsic::Compare(_) => MirTy::new(MirTyKind::Bool),
            BinaryIntrinsic::Float(_) => MirTy::new(MirTyKind::Float),
            BinaryIntrinsic::Integer(_) => MirTy::new(MirTyKind::Integer),
            BinaryIntrinsic::Boolean(_) => MirTy::new(MirTyKind::Bool),
            BinaryIntrinsic::String(_) => MirTy::new(MirTyKind::String),
        };

        let [lhs, rhs] = &args[..] else { panic!() };

        Immediate {
            kind: Box::new(ImmediateKind::Binary(binary_kind, lhs.clone(), rhs.clone())),
            ty,
            span,
        }
    }

    fn lower_unary_builtin(
        &self,
        builtin_name: &str,
        args: Vec<Immediate>,
        span: Span,
    ) -> Immediate {
        let (imm, ty) = match builtin_name {
            "not" => (UnaryIntrinsic::Not, MirTyKind::Bool),
            "bitnot" => (UnaryIntrinsic::BitNot, MirTyKind::Integer),

            "len" => (UnaryIntrinsic::Len, MirTyKind::Integer),

            "print" => (UnaryIntrinsic::Print, MirTyKind::Void),

            "parse_int" => (UnaryIntrinsic::Parse, MirTyKind::Integer),
            "format_int" => (UnaryIntrinsic::Format, MirTyKind::String),

            "parse_bool" => (UnaryIntrinsic::Parse, MirTyKind::Bool),
            "format_bool" => (UnaryIntrinsic::Format, MirTyKind::String),

            "parse_float" => (UnaryIntrinsic::Parse, MirTyKind::Float),
            "format_float" => (UnaryIntrinsic::Format, MirTyKind::String),

            "floor" => (UnaryIntrinsic::Floor, MirTyKind::Integer),
            "ceil" => (UnaryIntrinsic::Ceil, MirTyKind::Integer),
            "to_float" => (UnaryIntrinsic::ToFloat, MirTyKind::Float),

            "identity" => (UnaryIntrinsic::Identity, MirTyKind::Integer),
            "identity_float" => (UnaryIntrinsic::Identity, MirTyKind::Float),
            "negate" => (UnaryIntrinsic::Negate, MirTyKind::Integer),
            "negate_float" => (UnaryIntrinsic::Negate, MirTyKind::Float),

            _ => panic!(),
        };

        let ty = MirTy::new(ty);

        Immediate {
            kind: Box::new(ImmediateKind::Unary(imm, args[0].clone())),
            ty,
            span,
        }
    }
}
