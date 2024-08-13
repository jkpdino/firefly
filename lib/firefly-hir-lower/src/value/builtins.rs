
use firefly_interpret::ir::{ty::{Ty as VirTy, TyKind as VirTyKind}, value::{BinaryIntrinsic, BooleanBinaryOp, Comparison, FloatBinaryOp, Immediate, ImmediateKind, IntegerBinaryOp, StringBinaryOp, UnaryIntrinsic}};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub(super) fn lower_builtin(&self, builtin_name: &str, args: Vec<Immediate>, span: Span) -> Immediate {
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
            BinaryIntrinsic::Compare(_) => VirTy::new(VirTyKind::Bool),
            BinaryIntrinsic::Float(_) => VirTy::new(VirTyKind::Float),
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

    fn lower_unary_builtin(&self, builtin_name: &str, args: Vec<Immediate>, span: Span) -> Immediate {
        let (imm, ty) =
        match builtin_name {
            "not" => (UnaryIntrinsic::Not, VirTyKind::Bool),
            "bitnot" => (UnaryIntrinsic::BitNot, VirTyKind::Integer),

            "len" => (UnaryIntrinsic::Len, VirTyKind::Integer),

            "print" => (UnaryIntrinsic::Print, VirTyKind::Void),

            "parse_int" => (UnaryIntrinsic::Parse, VirTyKind::Integer),
            "format_int" => (UnaryIntrinsic::Format, VirTyKind::String),

            "parse_bool" => (UnaryIntrinsic::Parse, VirTyKind::Bool),
            "format_bool" => (UnaryIntrinsic::Format, VirTyKind::String),

            "parse_float" => (UnaryIntrinsic::Parse, VirTyKind::Float),
            "format_float" => (UnaryIntrinsic::Format, VirTyKind::String),

            "floor" => (UnaryIntrinsic::Floor, VirTyKind::Integer),
            "ceil" => (UnaryIntrinsic::Ceil, VirTyKind::Integer),
            "to_float" => (UnaryIntrinsic::ToFloat, VirTyKind::Float),

            _ => panic!(),
        };

        let ty = VirTy::new(ty);

        Immediate {
            kind: Box::new(ImmediateKind::Unary(imm, args[0].clone())),
            ty,
            span
        }
    }
}