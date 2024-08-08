
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InnerValue {
    Integer(u64),
    String(String),
    Boolean(bool),

    Tuple(Vec<Value>),

    Void,

    Undefined,
}

pub type Value = Box<InnerValue>;