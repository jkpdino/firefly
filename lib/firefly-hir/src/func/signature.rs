use itertools::Itertools;

use crate::{ty::{Ty, TyKind}, Name};

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


impl Callable {
    pub fn ty(&self) -> Ty {
        let params = self.params.iter()
                                .map(|p| &p.ty)
                                .cloned()
                                .collect_vec();
        let kind = TyKind::Func(params, Box::new(self.return_ty.clone()));

        Ty::new_unspanned(kind)
    }
}