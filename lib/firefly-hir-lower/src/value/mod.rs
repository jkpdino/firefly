mod builtins;
mod conditional;
mod literals;
pub mod loops;

use firefly_hir::{
    ty::TyKind,
    value::{LiteralValue, Value, ValueKind},
};
use firefly_mir::{
    code::Terminator,
    ty::{Ty as MirTy, TyKind as MirTyKind},
    value::{Immediate, ImmediateKind, Place, PlaceKind},
};
use itertools::Itertools;

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_immediate(&mut self, value: &Value) -> Immediate {
        match &value.kind {
            ValueKind::Literal(LiteralValue::Integer(num)) => self.lower_integer(num, value.span),
            ValueKind::Literal(LiteralValue::String(string)) => {
                self.lower_string(string, value.span)
            }
            ValueKind::Literal(LiteralValue::Boolean(boolean)) => {
                self.lower_bool(*boolean, value.span)
            }
            ValueKind::Literal(LiteralValue::Float(float)) => self.lower_float(float, value.span),

            ValueKind::Tuple(items) => self.lower_tuple(items, value.span),

            ValueKind::Unit => Immediate::void(),

            ValueKind::Invoke(function, args) => self.lower_call(function, args),
            ValueKind::Assign(place, value) => self.lower_assign(place, value),

            ValueKind::Return(value) => self.lower_return(value),
            ValueKind::Break(code_block) => self.lower_break(*code_block),
            ValueKind::Continue(code_block) => self.lower_continue(*code_block),

            ValueKind::If(if_value) => self.lower_if(if_value),

            ValueKind::While(while_value) => self.lower_while(while_value),

            ValueKind::StaticFunc(_)
            | ValueKind::DefaultInit(_)
            | ValueKind::InstanceFunc(..)
            | ValueKind::BuiltinFunc(_) => {
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
                // todo: globals dont work
                let mir_global = self.global_map[id];
                let global = self.hir.get(*id);

                Place {
                    kind: Box::new(PlaceKind::Global(mir_global)),
                    ty: self.lower_ty(&global.ty),
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

            other => todo!("internal compiler error: can't lower {other:?}"),
        }
    }

    pub fn lower_call(&mut self, func: &Value, args: &Vec<Value>) -> Immediate {
        let mut args = args
            .iter()
            .map(|arg| self.lower_immediate(arg))
            .collect_vec();

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
            ValueKind::InstanceFunc(receiver, instance_func) => {
                let instance_func = self.func_map[instance_func];

                // For methods, pass a copy of the receiver instead of moving it
                // The method will return the modified struct
                let receiver_place = self.lower_place(&receiver);
                let receiver_copy = receiver_place.clone().move_out();

                args.insert(0, receiver_copy);

                Immediate {
                    kind: Box::new(ImmediateKind::Call(instance_func, args)),
                    ty: receiver_place.ty.clone(), // Return type is the struct type
                    span: func.span,
                }
            }
            ValueKind::DefaultInit(struct_def) => {
                let struct_def = self.struct_map[struct_def];
                let ty = MirTy::new(MirTyKind::Struct(struct_def));

                Immediate {
                    kind: Box::new(ImmediateKind::Struct(args)),
                    ty,
                    span: func.span,
                }
            }
            ValueKind::Init(ty, init) => {
                let ty = self.lower_ty(ty);
                
                // Call the initializer function with the arguments and return the result directly
                let instance_func = self.init_map[init];

                Immediate {
                    kind: Box::new(ImmediateKind::Call(instance_func, args)),
                    ty,
                    span: func.span,
                }
            }
            ValueKind::BuiltinFunc(builtin_name) => {
                self.lower_builtin(builtin_name, args, func.span)
            }

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
        } else {
            self.mir.build_terminator(Terminator::returns(imm));
        }

        Immediate::void()
    }
}
