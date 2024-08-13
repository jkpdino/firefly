use firefly_hir::{func::Callable, resolve::Symbol, stmt::CodeBlock, Id};
use firefly_mir::code::Terminator;

use crate::HirLowerer;
use firefly_hir::func::Func as HirFunc;

impl HirLowerer<'_> {
    pub fn create_func(&mut self, func: Id<HirFunc>) {
        let Symbol { name, .. } = self.hir.try_get(func)
            .expect("internal compiler error: function doesn't have a symbol");

        let Callable { params, return_ty, .. } = self.hir.try_get(func)
            .expect("internal compiler error: function doesn't have a signature");

        let mir_params = params.iter().map(|p| self.lower_ty(&p.ty)).collect();
        let return_ty = self.lower_ty(return_ty);

        let mir_id = self.mir.context_mut().create_function(&name.name, mir_params, return_ty);

        for param in params {
            let ty = self.lower_ty(&param.ty);
            let mir_local = self.mir.context_mut().create_local(mir_id, ty);
            self.local_map.insert(param.id, mir_local.id());
        }

        self.func_map.insert(func, mir_id);
    }

    pub fn lower_func(&mut self, func: Id<HirFunc>) {
        let mir_id = *self.func_map.get(&func).unwrap();

        self.mir.select_func(mir_id);

        let Some(code_block) = self.hir.children(func.as_base()).iter().find_map(|child| self.hir.cast_id::<CodeBlock>(*child)) else {
            return;
        };

        // Lower the code
        let bb0 = self.mir.append_basic_block();
        self.mir.select_basic_block(bb0);
        self.lower_code_block(code_block);

        // If the basic block isn't terminated, return void
        if !self.mir.is_terminated() {
            self.mir.build_terminator(Terminator::returns_void());
        }
    }
}