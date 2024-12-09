#[derive(Copy, Clone, Debug)]
pub enum PrefixOperator {
  Identity,
  Invert,
  Negate,
}

#[derive(Copy, Clone, Debug)]
pub enum InfixOperator {
  Add,
  Subtract,
  Multiply,
  Divide,
  Modulo,
  ShiftLeft,
  ShiftRight,
  BitAnd,
  BitXor,
  BitOr,
  CompareLessThan,
  CompareGreaterThan,
  CompareLessThanOrEqual,
  CompareGreaterThanOrEqual,
  CompareEqual,
  CompareNotEqual,
  LogicalAnd,
  LogicalOr,

  AddAssign,
  SubtractAssign,
  MultiplyAssign,
  DivideAssign,
  ModuloAssign,
  ShiftLeftAssign,
  ShiftRightAssign,
  BitAndAssign,
  BitOrAssign,
  BitXorAssign,

  Assign,
}

impl PrefixOperator {
  pub fn get_verb(&self) -> &'static str {
    match self {
        PrefixOperator::Identity => "identity",
        PrefixOperator::Invert => "invert",
        PrefixOperator::Negate => "negate",
    }
  }
}

impl InfixOperator {
  pub fn get_verb(&self) -> &'static str {
    match self {
        InfixOperator::Add => "add",
        InfixOperator::Subtract => "subtract",
        InfixOperator::Multiply => "multiply",
        InfixOperator::Divide => "divide",
        InfixOperator::Modulo => "modulo",
        InfixOperator::ShiftLeft => "shiftLeft",
        InfixOperator::ShiftRight => "shiftRight",
        InfixOperator::BitAnd => "bitAnd",
        InfixOperator::BitXor => "bitXor",
        InfixOperator::BitOr => "bitOr",
        InfixOperator::CompareLessThan => "lessThan",
        InfixOperator::CompareGreaterThan => "greaterThan",
        InfixOperator::CompareLessThanOrEqual => "lessThanEq",
        InfixOperator::CompareGreaterThanOrEqual => "greaterThanEq",
        InfixOperator::CompareEqual => "equals",
        InfixOperator::CompareNotEqual => "notEquals",
        InfixOperator::LogicalAnd => "logicalAnd",
        InfixOperator::LogicalOr => "logicalOr",
        InfixOperator::AddAssign => "addAssign",
        InfixOperator::SubtractAssign => "subAssign",
        InfixOperator::MultiplyAssign => "multiplyAssign",
        InfixOperator::DivideAssign => "divideAssign",
        InfixOperator::ModuloAssign => "moduloAssign",
        InfixOperator::ShiftLeftAssign => "shiftLeftAssign",
        InfixOperator::ShiftRightAssign => "shiftRightAssign",
        InfixOperator::BitAndAssign => "bitAndAssign",
        InfixOperator::BitOrAssign => "bitOrAssign",
        InfixOperator::BitXorAssign => "bitXorAssign",
        InfixOperator::Assign => "assign"
    }
  }
}