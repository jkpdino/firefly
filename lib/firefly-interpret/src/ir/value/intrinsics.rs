use std::fmt::Display;

#[derive(Clone)]
pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Clone)]
pub enum IntegerBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    ShiftLeft,
    ShiftRight,
    BitOr,
    BitAnd,
    BitXor,
}

#[derive(Clone)]
pub enum IntegerUnaryOp {
    BitNot,
}

#[derive(Clone)]
pub enum BooleanBinaryOp {
    Or,
    And,
    Xor,
}

#[derive(Clone)]
pub enum StringBinaryOp {
    Concat,
}

#[derive(Clone)]
pub enum BinaryIntrinsic {
    Compare(Comparison),

    Integer(IntegerBinaryOp),
    Boolean(BooleanBinaryOp),
    String(StringBinaryOp)
}

pub enum UnaryIntrinsic {
    BitNot,

    Not,

    Print,

    Format,
    Parse,
}

impl Display for Comparison {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparison::Equal => write!(f, "equal"),
            Comparison::NotEqual => write!(f, "not_equal"),
            Comparison::LessThan => write!(f, "less_than"),
            Comparison::LessThanOrEqual => write!(f, "less_than_or_equal"),
            Comparison::GreaterThan => write!(f, "greater_than"),
            Comparison::GreaterThanOrEqual => write!(f, "greater_than_or_equal"),
        }
    }
}

impl Display for IntegerBinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IntegerBinaryOp::Add => write!(f, "add"),
            IntegerBinaryOp::Sub => write!(f, "sub"),
            IntegerBinaryOp::Mul => write!(f, "mul"),
            IntegerBinaryOp::Div => write!(f, "div"),
            IntegerBinaryOp::Rem => write!(f, "rem"),
            IntegerBinaryOp::ShiftLeft => write!(f, "shift_left"),
            IntegerBinaryOp::ShiftRight => write!(f, "shift_right"),
            IntegerBinaryOp::BitOr => write!(f, "bit_or"),
            IntegerBinaryOp::BitAnd => write!(f, "bit_and"),
            IntegerBinaryOp::BitXor => write!(f, "bit_xor"),
        }
    }
}

impl Display for BooleanBinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BooleanBinaryOp::And => write!(f, "and"),
            BooleanBinaryOp::Or => write!(f, "or"),
            BooleanBinaryOp::Xor => write!(f, "xor"),
        }
    }
}

impl Display for StringBinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StringBinaryOp::Concat => write!(f, "concat")
        }
    }
}

impl Display for BinaryIntrinsic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryIntrinsic::Compare(comparison) => write!(f, "compare[{comparison}]"),
            BinaryIntrinsic::Integer(op) => write!(f, "{op}"),
            BinaryIntrinsic::Boolean(op) => write!(f, "{op}"),
            BinaryIntrinsic::String(op) => write!(f, "{op}"),
        }
    }
}