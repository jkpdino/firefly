use super::value::{InnerValue, Value};

pub struct StackFrame {
    frame: Vec<Value>
}

impl StackFrame {
    pub fn new(size: usize, params: Vec<Value>) -> Self {
        assert!(size >= params.len());
        
        let param_len = params.len();

        let mut frame = params;

        for _ in 0..size - param_len {
            frame.push(Value::new(InnerValue::Undefined));
        }

        Self { frame }
    }

    pub fn get_value(&self, n: usize) -> &Value {
        self.frame.get(n).expect("internal error: stack frame access out of bounds")
    }

    pub fn get_value_mut(&mut self, n: usize) -> &mut Value {
        self.frame.get_mut(n).expect("internal error: stack frame access out of bounds")
    }
}