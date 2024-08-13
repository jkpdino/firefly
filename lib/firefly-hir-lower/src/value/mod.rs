pub mod loops;
mod literals;
mod builtins;
mod conditional;

use firefly_hir::{ty::TyKind, value::{LiteralValue, Value, ValueKind}};
use firefly_mir::{code::Terminator, value::{Immediate, ImmediateKind, Place, PlaceKind}};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_immediate(&mut self, value: &Value) -> Immediate {
        match &value.kind {
            ValueKind::Literal(LiteralValue::Integer(num)) => self.lower_integer(num, value.span),
            ValueKind::Literal(LiteralValue::String(string)) => self.lower_string(string, value.span),
            ValueKind::Literal(LiteralValue::Boolean(boolean)) => self.lower_bool(*boolean, value.span),
            ValueKind::Literal(LiteralValue::Float(float)) => self.lower_float(float, value.span),

            ValueKind::Tuple(items) => self.lower_tuple(items, value.span),

            ValueKind::Unit => Immediate::void(),

            ValueKind::Invoke(function, args) => self.lower_call(function, args),
            ValueKind::Assign(place, value) => self.lower_assign(place, value),

            ValueKind::Return(value) => self.lower_return(value),
            ValueKind::Break(code_block) => self.lower_break(*code_block),
            ValueKind::Continue(code_block) => self.lower_continue(*code_block),

            ValueKind::If(if_value) => self.lower_if(if_value, None),

            ValueKind::While(while_value) => self.lower_while(while_value),

            ValueKind::StaticFunc(_) | ValueKind::InitFor(_) | ValueKind::InstanceFunc(..) | ValueKind::BuiltinFunc(_) => {
                panic!("internal compiler error: first-class functions are not supported yet");
            }

            _ => self.lower_place(value).move_out(),
        }
    }

    pub fn lower_place(&mut self, value: &Value) -> Place {
        match &value.kind {
            ValueKind::Local(id) => {
                let mir_local = self.local_map[id];
                let local = self.hir.get(*id);

                Place {
                    kind: Box::new(PlaceKind::Local(mir_local)),
                    ty: self.lower_ty(&local.ty),
                    span: value.span,
                }
            }

            ValueKind::Global(id) => {
                let mir_global = self.global_map[id];
                let global = self.hir.get(*id);

                Place {
                    kind: Box::new(PlaceKind::Global(mir_global)),
                    ty: self.lower_ty(&global.ty),
                    span: value.span
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

            other => todo!("internal compiler error: can't lower {other:?}"),
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

    fn lower_assign(&mut self, place: &Value, value: &Value) -> Immediate {
        let place = self.lower_place(place);
        let value = self.lower_immediate(value);

        self.mir.build_assign(place, value);

        Immediate::void()
    }

    fn lower_return(&mut self, value: &Value) -> Immediate {
        let imm = self.lower_immediate(value);

        if let ImmediateKind::Void = imm.kind.as_ref() {
            self.mir.build_terminator(Terminator::returns_void());
        }
        else {
            self.mir.build_terminator(Terminator::returns(imm));
        }


        Immediate::void()
    }
}