use super::value::Value;

#[allow(dead_code)]
pub struct StackFrame {
    frame: Vec<Value>
}

impl StackFrame {
    #[allow(dead_code, unused_variables)]
    pub fn new(size: usize) -> Self {
        todo!()
    }

    #[allow(dead_code, unused_variables)]
    pub fn get_value(&self, n: usize) -> &Value {
        todo!()
    }
}