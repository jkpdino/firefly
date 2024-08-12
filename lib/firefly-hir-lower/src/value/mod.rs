use firefly_hir::{ty::TyKind, value::{ElseValue, IfValue, LiteralValue, Value, ValueKind}};
use firefly_interpret::ir::{code::{BasicBlockId, Terminator, TerminatorKind}, ty::{Ty as VirTy, TyKind as VirTyKind}, value::{BinaryIntrinsic, BooleanBinaryOp, Comparison, ConstantValue, FloatBinaryOp, Immediate, ImmediateKind, IntegerBinaryOp, Place, PlaceKind, StringBinaryOp, UnaryIntrinsic}};
use firefly_span::Span;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_immediate(&mut self, value: &Value) -> Immediate {
        match &value.kind {
            ValueKind::Literal(LiteralValue::Integer(num)) => self.lower_integer(num, value.span),
            ValueKind::Literal(LiteralValue::String(string)) => self.lower_string(string, value.span),
            ValueKind::Literal(LiteralValue::Boolean(boolean)) => self.lower_bool(*boolean, value.span),
            ValueKind::Literal(LiteralValue::Float(float)) => self.lower_float(float, value.span),

            ValueKind::Invoke(function, args) => self.lower_call(function, args),

            ValueKind::Assign(place, value) => {
                let place = self.lower_place(place);
                let value = self.lower_immediate(value);

                self.vir.build_assign(place, value);

                Immediate::void()
            }

            ValueKind::Return(value) => {
                let imm = self.lower_immediate(value);

                self.vir.build_terminator(Terminator {
                    kind: TerminatorKind::Return(imm)
                });

                Immediate::void()
            }

            ValueKind::If(if_value) => {
                self.lower_if(if_value, None);

                Immediate::void()
            }

            _ => self.lower_place(value).move_out(),
        }
    }

    pub fn lower_place(&mut self, value: &Value) -> Place {
        // Globals
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

            ValueKind::FieldOf(place, field) => {
                let place = self.lower_place(place);
                let field = self.field_map[field];

                Place {
                    kind: Box::new(PlaceKind::Field(place, field)),
                    ty: self.lower_ty(&value.ty),
                    span: value.span,
                }
            }

            ValueKind::TupleMember(tuple, index) => {
                let tuple = self.lower_place(tuple);

                Place {
                    kind: Box::new(PlaceKind::Field(tuple, *index)),
                    ty: self.lower_ty(&value.ty),
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

    fn lower_float(&self, float: &str, span: Span) -> Immediate {
        let float = float.parse().unwrap_or(f64::NAN);

        Immediate {
            kind: Box::new(ImmediateKind::Constant(ConstantValue::Float(float))),
            ty: VirTy::new(VirTyKind::Float),
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

    fn lower_if(&mut self, if_value: &IfValue, after_block: Option<BasicBlockId>) {
        let condition = self.lower_immediate(&if_value.condition);

        let then_block = self.vir.append_basic_block();
        let else_block = self.vir.append_basic_block();

        let after_block = after_block.unwrap_or_else(|| self.vir.append_basic_block());

        self.vir.build_terminator(Terminator {
            kind: TerminatorKind::BranchIf(condition, then_block, else_block)
        });

        self.vir.select_basic_block(then_block);
        self.lower_code_block(if_value.positive);
        self.vir.build_terminator(Terminator {
            kind: TerminatorKind::Branch(after_block)
        });

        self.vir.select_basic_block(else_block);
        match &if_value.negative {
            Some(ElseValue::Else(code_block)) => self.lower_code_block(*code_block),
            Some(ElseValue::ElseIf(if_value)) => self.lower_if(if_value, Some(after_block)),

            None => {}
        }
        self.vir.build_terminator(Terminator {
            kind: TerminatorKind::Branch(after_block)
        });

        self.vir.select_basic_block(after_block);
    }
}