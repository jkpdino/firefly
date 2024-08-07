use firefly_hir::{func::Callable, resolve::Symbol, stmt::CodeBlock, Id};

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

        let vir_id = self.vir.context_mut().create_function(&name.name, params, return_ty);

        self.func_map.insert(func, vir_id);
    }

    pub fn lower_func(&mut self, func: Id<HirFunc>) {
        let vir_id = *self.func_map.get(&func).unwrap();

        self.vir.select_func(vir_id);

        let Some(code_block) = self.hir.children(func.as_base()).iter().find_map(|child| self.hir.cast_id::<CodeBlock>(*child)) else {
            return;
        };

        let bb0 = self.vir.append_basic_block();
        self.vir.select_basic_block(bb0);


        self.lower_code_block(code_block);
    }
}