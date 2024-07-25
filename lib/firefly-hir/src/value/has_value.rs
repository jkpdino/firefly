use super::Value;

#[derive(Debug, Clone)]
pub struct HasValue {
    pub value: Value
}

component!(has_values: HasValue);