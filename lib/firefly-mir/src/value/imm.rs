use std::fmt::{Display, Formatter};

use firefly_span::Span;
use itertools::Itertools;

use crate::{code::Function, ty::{Ty, TyKind}, util::Id};

use super::{intrinsics::BinaryIntrinsic, Place, UnaryIntrinsic};

#[derive(Debug, Clone, PartialEq)]
pub enum ConstantValue {
    Integer(u64),
    Bool(bool),
    Float(f64),
    String(String),
}

#[derive(Clone)]
pub enum ImmediateKind {
    /// A constant immediate
    Constant(ConstantValue),

    /// Takes the value currently in a place
    Move(Place),

    /// Calls a function
    Call(Id<Function>, Vec<Immediate>),

    /// Performs an intrinsic operation on two immediates 
    Binary(BinaryIntrinsic, Immediate, Immediate),

    /// Performs a unary operation on an immediate
    Unary(UnaryIntrinsic, Immediate),

    Void,
}

#[derive(Clone)]
pub struct Immediate {
    pub kind: Box<ImmediateKind>,
    pub ty:   Ty,
    pub span: Span,
}

impl Immediate {
    pub fn void() -> Self {
        Immediate { kind: Box::new(ImmediateKind::Void), ty: Ty::new(TyKind::Void), span: Span::default() }
    }
}

impl Display for ConstantValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstantValue::Integer(integer) => write!(f, "{integer}"),
            ConstantValue::Bool(boolean) => write!(f, "{boolean}"),
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
            ImmediateKind::Call(function, args) => write!(f, "invoke {function:?} ({})", args.iter().format(", ")),
            ImmediateKind::Binary(func, left, right) => write!(f, "{func} ({left}, {right})"),
            ImmediateKind::Unary(func, operand) => write!(f, "{func} ({operand})"),
            ImmediateKind::Void => write!(f, "void")
        }
    }
}

impl Display for Immediate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}