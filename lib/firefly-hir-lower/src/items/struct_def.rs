use firefly_hir::{items::Field, resolve::Symbol, Id};

use crate::HirLowerer;
use firefly_hir::items::StructDef as HirStructDef;

impl HirLowerer<'_> {
    pub fn create_struct(&mut self, struct_def: Id<HirStructDef>) {
        let Symbol { name, .. } = self.hir.try_get(struct_def)
            .expect("internal compiler error: struct doesn't have a symbol");

        let vir_id = self.vir.context_mut().create_struct(name.name.clone());

        self.struct_map.insert(struct_def, vir_id);
    }

    pub fn lower_struct(&mut self, struct_def: Id<HirStructDef>) {
        let vir_id = *self.struct_map.get(&struct_def).unwrap();

        let fields = self.hir.children(struct_def.as_base())
            .iter()
            .filter_map(|field| self.hir.cast_id::<Field>(*field));

        for field in fields {
            let field = self.hir.get(field);

            let field_ty = self.lower_ty(&field.ty);

            self.vir.context_mut().create_field(vir_id, field_ty);
        }
    }
}