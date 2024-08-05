use firefly_hir::{resolve::Symbol, Id};

use crate::HirLowerer;
use firefly_hir::items::StructDef as HirStructDef;

impl HirLowerer<'_> {
    pub fn create_struct(&mut self, struct_def: Id<HirStructDef>) {
        let Symbol { name, .. } = self.hir.try_get(struct_def)
            .expect("internal compiler error: struct doesn't have a symbol");

        let vir_id = self.vir.create_struct(name.name.clone());

        self.struct_map.insert(struct_def, vir_id);
    }
}