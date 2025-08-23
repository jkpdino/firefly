use firefly_hir::{func::Callable, items::mangle::MangledName, stmt::CodeBlock, value::HasSelf, Id};
use itertools::Itertools;

use crate::HirLowerer;
use firefly_hir::func::Func as HirFunc;

impl HirLowerer<'_> {
    pub fn create_func(&mut self, func: Id<HirFunc>) {
        let MangledName { symbol } = self.hir.try_get_computed(func).cloned()
            .expect("internal compiler error: function doesn't have a mangled name");

        let Callable { params, return_ty, .. } = self.hir.try_get(func)
            .expect("internal compiler error: function doesn't have a signature");



        // create the function
        let mut mir_params = params.iter().map(|p| self.lower_ty(&p.ty)).collect_vec();
        let mut return_ty = self.lower_ty(return_ty);

        if let Some(HasSelf { ty, .. }) = self.hir.try_get::<HasSelf>(func) {
            let self_ty = self.lower_ty(ty);

            mir_params.insert(0, self_ty.clone());
            
            // If this is a method (has self parameter), make it return the self type
            // so that mutations can be persisted by returning the modified struct
            return_ty = self_ty;
        }

        let mir_id = self.mir.context_mut().create_function(&symbol, mir_params, return_ty);

        // add the self parameter to the function
        if let Some(HasSelf { local, ty }) = self.hir.try_get::<HasSelf>(func) {
            let ty = self.lower_ty(ty);
            let mir_local = self.mir.context_mut().create_local(mir_id, ty);
            self.local_map.insert(*local, mir_local.id());
        }

        // add locals to the function
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

        // Check if this is a method (has self parameter)
        let has_self = self.hir.try_get::<HasSelf>(func);
        
        if let Some(HasSelf { local, .. }) = has_self {
            // For methods, use lower_code_block_inner and add explicit return of self
            let self_local = self.local_map[&local];
            let _code_result = self.lower_code_block_inner(code_block);

            use firefly_mir::code::Terminator;
            
            let self_place = self.mir.get_local(self_local).place_unspanned();
            let return_value = self_place.move_out();
            
            self.mir.build_terminator(Terminator::returns(return_value));
        } else {
            // For regular functions, use the normal flow
            self.lower_code_block_func(code_block);
        }
    }
}