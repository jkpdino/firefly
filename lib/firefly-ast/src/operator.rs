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
  pub fn precedence(&self) -> u32 {
    match self {
        // Additive (500)
        InfixOperator::Add => 500,
        InfixOperator::Subtract => 500,
        InfixOperator::BitOr => 500,
        InfixOperator::BitXor => 500,
        
        // Multiplicative (600)
        InfixOperator::Multiply => 600,
        InfixOperator::Divide => 600,
        InfixOperator::Modulo => 600,
        InfixOperator::BitAnd => 600,
        
        // Exponentitive (700)
        InfixOperator::ShiftLeft => 700,
        InfixOperator::ShiftRight => 700,
        
        // Relational (400)
        InfixOperator::CompareLessThan => 400,
        InfixOperator::CompareGreaterThan => 400,
        InfixOperator::CompareLessThanOrEqual => 400,
        InfixOperator::CompareGreaterThanOrEqual => 400,
        InfixOperator::CompareEqual => 400,
        InfixOperator::CompareNotEqual => 400,
        
        // Logical (300, 200)
        InfixOperator::LogicalAnd => 300,
        InfixOperator::LogicalOr => 200,
        
        // Assignment (100)
        InfixOperator::Assign => 100,
        InfixOperator::AddAssign => 100,
        InfixOperator::SubtractAssign => 100,
        InfixOperator::MultiplyAssign => 100,
        InfixOperator::DivideAssign => 100,
        InfixOperator::ModuloAssign => 100,
        InfixOperator::ShiftLeftAssign => 100,
        InfixOperator::ShiftRightAssign => 100,
        InfixOperator::BitAndAssign => 100,
        InfixOperator::BitOrAssign => 100,
        InfixOperator::BitXorAssign => 100,
    }
}
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