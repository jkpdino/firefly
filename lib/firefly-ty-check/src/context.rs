use firefly_hir::{ty::Ty, value::Value};

use crate::errors::TypeCheckError;

pub struct TypecheckContext {

}

impl TypecheckContext {
    pub fn can_assign_to(&self, ty: &Ty, value: &Value) -> bool {
        false
    }

    pub fn throw(&self, error: TypeCheckError) {
        
    }
}