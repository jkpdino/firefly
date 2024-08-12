
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InnerValue {
    Integer(u64),
    String(String),
    Boolean(bool),
    Float(f64),

    Tuple(Vec<Value>),

    Void,

    Undefined,
}

pub type Value = Box<InnerValue>;