use super::value::Value;

pub struct StackFrame {
    frame: Vec<Value>
}

impl StackFrame {
    pub fn new(size: usize) -> Self {
        todo!()
    }

    pub fn get_value(&self, n: usize) -> &Value {
        todo!()
    }
}