use firefly_hir::{items::{mangle::MangledName, Global as HirGlobal}, Id};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn create_global(&mut self, id: Id<HirGlobal>) {
        let MangledName { symbol } = self.hir.try_get_computed(id).cloned()
            .expect("internal compiler error: function doesn't have a mangled name");

        let global = self.hir.get(id);

        let global_ty = self.lower_ty(&global.ty);

        self.mir.context_mut().create_global(&symbol, global_ty);
    }
}