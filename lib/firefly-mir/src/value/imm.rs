use std::fmt::{Display, Formatter};

use firefly_span::Span;
use itertools::Itertools;

use crate::{code::Function, ty::{Ty, TyKind}, util::Id, DisplayInContext, MirContext};

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

    /// Constructs a tuple
    Tuple(Vec<Immediate>),

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

impl DisplayInContext for ImmediateKind {
    fn fmt(&self, f: &mut Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        match self {
            ImmediateKind::Constant(constant) => write!(f, "const {constant}"),
            ImmediateKind::Tuple(items) => write!(f, "tuple ({})", items.iter().map(|item| context.display(item)).format(", ")),
            ImmediateKind::Move(place) => write!(f, "move {place}"),
            ImmediateKind::Call(function, args) => {
                let func = context.get_function(*function);

                let func_name = &func.name;

                write!(f, "invoke {func_name} ({})", args.iter().map(|arg| context.display(arg)).format(", "))
            }
            ImmediateKind::Binary(func, left, right) => write!(f, "{func} ({}, {})", context.display(left), context.display(right)),
            ImmediateKind::Unary(func, operand) => write!(f, "{func} ({})", context.display(operand)),
            ImmediateKind::Void => write!(f, "void")
        }
    }
}

impl DisplayInContext for Immediate {
    fn fmt(&self, f: &mut Formatter<'_>, context: &MirContext) -> std::fmt::Result {
        self.kind.fmt(f, context)
    }
}