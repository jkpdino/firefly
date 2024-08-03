use std::fmt::Debug;

use crate::{func::Func, items::Field, stmt::Local, ty::Ty, Id};

use super::Value;

#[derive(Debug, Clone)]
pub struct HasValue {
    pub value: Value
}

component!(has_values: HasValue);

#[derive(Debug, Clone)]
pub enum HasValueIn {
    Field(Id<Field>),
    Method(Id<Func>),
}

component!(has_values_in: HasValueIn);

#[derive(Debug, Clone)]
pub struct HasSelf {
    pub local: Id<Local>,
    pub ty: Ty,
}

component!(has_self: HasSelf);