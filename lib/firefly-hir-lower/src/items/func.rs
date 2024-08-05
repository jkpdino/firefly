use firefly_hir::{func::Callable, resolve::Symbol, Id};

use crate::HirLowerer;
use firefly_hir::func::Func as HirFunc;

impl HirLowerer<'_> {
    pub fn create_func(&mut self, func: Id<HirFunc>) {
        let Symbol { name, .. } = self.hir.try_get(func)
            .expect("internal compiler error: function doesn't have a symbol");

        let Callable { params, return_ty } = self.hir.try_get(func)
            .expect("internal compiler error: function doesn't have a signature");

        let params = params.iter().map(|p| self.lower_ty(&p.ty)).collect();
        let return_ty = self.lower_ty(return_ty);

        let vir_id = self.vir.create_function(&name.name, params, return_ty);

        self.func_map.insert(func, vir_id);
    }
}