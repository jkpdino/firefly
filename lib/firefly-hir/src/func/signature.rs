use crate::{ty::Ty, Name};

/// Marks a symbol as callable and provides a signature
/// for calling the symbol
#[derive(Debug, Clone)]
pub struct Callable {
    pub params: Vec<FuncParam>,
    pub return_ty: Ty,
}

#[derive(Debug, Clone)]
pub struct FuncParam {
    pub bind_name: Name,
    pub ty: Ty,
}

component!(callables: Callable);


