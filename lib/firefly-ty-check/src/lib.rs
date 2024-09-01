use context::TypecheckContext;
use errors::TypeCheckError;
use firefly_hir::{stmt::{Stmt, StmtKind}, ty::Ty, value::{IfValue, Value, ValueKind}};

mod context;
mod errors;

pub struct Typecheck<'a> {
    return_type: Option<Ty>,

    context: &'a TypecheckContext,
}

impl Typecheck<'_> {
    pub fn typecheck_statement(&self, stmt: &Stmt) {
        match &stmt.kind {
            StmtKind::Bind(_, ty, value) => {
                self.typecheck_bind(ty, value);
            }

            _ => {}
        }
    }

    pub fn typecheck_value(&self, value: &Value) {
        /*match &value.kind {
            ValueKind::Assign(lhs, rhs) => {
                self.context.expect_eq(rhs, lhs.ty);
            }
            ValueKind::Return(value) => {
                // check for return type
            }
            ValueKind::If(if_value) => {
                self.context.expect_bool(&if_value.condition);
            }
            ValueKind::While(while_value) => {
                expect_value_of_type(&while_value.condition, &IfValue);
            }
            ValueKind::Invoke(_, _) => todo!(),
        }*/
    }

    fn typecheck_bind(&self, ty: &Ty, value: &Value) {
        if self.context.can_assign_to(ty, value) {
            return;
        }

        self.context.throw(TypeCheckError::BindingTypeMismatch(ty, value))
    }
}