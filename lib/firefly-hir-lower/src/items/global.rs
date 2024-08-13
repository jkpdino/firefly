use firefly_hir::{items::Global as HirGlobal, resolve::Symbol, Id};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn create_global(&mut self, id: Id<HirGlobal>) {
        let global = self.hir.get(id);

        let Some(Symbol { name, .. }) = self.hir.try_get(id) else {
            panic!("internal compiler error: expected global to have a symbol");
        };

        let global_ty = self.lower_ty(&global.ty);

        self.mir.context_mut().create_global(&name.name, global_ty);
    }
}