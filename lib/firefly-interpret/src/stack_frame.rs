use firefly_mir::{ty::Ty, MirContext};

use crate::value::default_for_ty;

use super::value::Value;

pub struct StackFrame {
    frame: Vec<Value>,
}

impl StackFrame {
    pub fn new(types: Vec<Ty>, params: Vec<Value>, context: &MirContext) -> Self {
        let size = types.len();
        assert!(size >= params.len());

        let param_len = params.len();

        let mut frame = params;

        for i in param_len..size {
            let ty = &types[i];

            frame.push(default_for_ty(ty, context));
        }

        Self { frame }
    }

    #[allow(dead_code)]
    pub fn get_value(&self, n: usize) -> &Value {
        self.frame
            .get(n)
            .expect("internal error: stack frame access out of bounds")
    }

    pub fn get_value_mut(&mut self, n: usize) -> &mut Value {
        self.frame
            .get_mut(n)
            .expect("internal error: stack frame access out of bounds")
    }
}
