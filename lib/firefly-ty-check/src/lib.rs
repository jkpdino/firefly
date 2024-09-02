use context::TypecheckContext;
use errors::TypeCheckError;
use firefly_hir::{stmt::{Stmt, StmtKind}, ty::{Ty, TyKind}, value::{ElseValue, IfValue, Value, ValueKind, WhileValue}};
use itertools::Itertools;

mod context;
mod errors;
pub mod pass;

pub struct Typecheck<'a, 'b> {
    return_type: Option<Ty>,

    context: &'a TypecheckContext<'b>,
}

impl Typecheck<'_, '_> {
    pub fn typecheck_statement(&self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Bind(_, _, ty, value) => {
                self.typecheck_bind(ty, value);
            }

            _ => {}
        }
    }

    pub fn typecheck_value(&self, value: &Value) {
        match &value.kind {
            ValueKind::Assign(lhs, rhs) => {
                self.typecheck_assign(lhs, rhs);
            }
            ValueKind::Return(value) => {
                self.typecheck_return(value);
            }
            ValueKind::If(if_value) => {
                self.typecheck_if(if_value);
            }
            ValueKind::While(while_value) => {
                self.typecheck_while(while_value);
            }
            ValueKind::Invoke(func, args) => {
                self.typecheck_invoke(func, args);
            },

            _ => {}
        }
    }

    fn typecheck_bind(&self, ty: &Ty, value: &Value) {
        if self.context.can_assign_to(ty, &value.ty) {
            return;
        }

        self.context.throw(TypeCheckError::BindingTypeMismatch(ty, value))
    }

    fn typecheck_assign(&self, lhs: &Value, rhs: &Value) {
        if self.context.can_assign_to(&lhs.ty, &rhs.ty) {
            return;
        }

        self.context.throw(TypeCheckError::MismatchedType(&lhs.ty, rhs))
    }

    fn typecheck_if(&self, if_value: &IfValue) {
        let boolean = Ty::new_unspanned(TyKind::Bool);

        if !self.context.can_assign_to(&boolean, &if_value.condition.ty) {
            self.context.throw(TypeCheckError::IfConditionBool(&if_value.condition));
        }

        match &if_value.negative {
            Some(ElseValue::ElseIf(else_if_value)) => self.typecheck_if(else_if_value),

            _ => {}
        }
    }

    fn typecheck_while(&self, while_value: &WhileValue) {
        let boolean = Ty::new_unspanned(TyKind::Bool);

        if self.context.can_assign_to(&boolean, &while_value.condition.ty) {
            return;
        }

        self.context.throw(TypeCheckError::WhileConditionBool(&while_value.condition));
    }

    fn typecheck_return(&self, value: &Value) {
        let Some(return_type) = &self.return_type else {
            return; // throw an error?
        };

        if self.context.can_assign_to(return_type, &value.ty) {
            return;
        }

        self.context.throw(TypeCheckError::WrongReturnType(return_type, value));
    }

    fn typecheck_invoke(&self, func: &Value, args: &[Value]) {
        let TyKind::Func(params, _) = &func.ty.kind else {
            panic!("internal compiler error: can't call a non-functoin");
        };

        if params.len() > args.len() {
            self.context.throw(TypeCheckError::MissingFunctionArgs(
                func,
                &params[args.len()..]
            ))
        }
        else if params.len() < args.len() {
            self.context.throw(TypeCheckError::ExtraFunctionArgs(
                &args[params.len()..]
            ))
        }

        let non_matching = params.iter().zip(args).filter(|(param, arg)| !self.context.can_assign_to(param, &arg.ty)).collect_vec();

        if non_matching.len() > 0 {
            self.context.throw(TypeCheckError::WrongFunctionArgs(&non_matching))
        }
    }
}