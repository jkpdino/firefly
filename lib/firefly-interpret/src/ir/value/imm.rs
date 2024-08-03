use std::fmt::{Display, Formatter};

use firefly_span::Span;
use itertools::Itertools;

use crate::ir::ty::Ty;

use super::{intrinsics::BinaryIntrinsic, Place};

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    Integer(u64),
    Float(f64),
    String(String),
}

pub enum ImmediateKind {
    /// A constant immediate
    Constant(ConstantValue),

    /// Takes the value currently in a place
    Move(Place),

    /// Gets a reference to a function
    Function(String),

    /// Calls a function
    Invoke(Immediate, Vec<Immediate>),

    /// Performs an intrinsic operation on two immediates 
    Binary(BinaryIntrinsic, Immediate, Immediate)
}

pub struct Immediate {
    kind: Box<ImmediateKind>,
    ty:   Ty,
    span: Span,
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantValue::Integer(integer) => write!(f, "{integer}"),
            ConstantValue::Float(float) => write!(f, "{float}"),
            ConstantValue::String(string) => write!(f, "{string}")
        }
    }
}

impl Display for ImmediateKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ImmediateKind::Constant(constant) => write!(f, "const {constant}"),
            ImmediateKind::Move(place) => write!(f, "move {place}"),
            ImmediateKind::Function(name) => write!(f, "{name}"),
            ImmediateKind::Invoke(function, args) => write!(f, "invoke {function} ({})", args.iter().format(", ")),
            ImmediateKind::Binary(func, left, right) => write!(f, "{func} ({left}, {right})")
        }
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}