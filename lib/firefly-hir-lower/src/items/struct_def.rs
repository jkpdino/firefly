use firefly_hir::{items::Field, resolve::Symbol, Id};

use crate::HirLowerer;
use firefly_hir::items::StructDef as HirStructDef;

impl HirLowerer<'_> {
    pub fn create_struct(&mut self, struct_def: Id<HirStructDef>) {
        let Symbol { name, .. } = self.hir.try_get(struct_def)
            .expect("internal compiler error: struct doesn't have a symbol");

        let mir_id = self.mir.context_mut().create_struct(name.name.clone());

        self.struct_map.insert(struct_def, mir_id);
    }

    pub fn lower_struct(&mut self, struct_def: Id<HirStructDef>) {
        let mir_id = *self.struct_map.get(&struct_def).unwrap();

        let fields = self.hir.children(struct_def.as_base())
            .iter()
            .filter_map(|field| self.hir.cast_id::<Field>(*field));

        for field_id in fields {
            let field = self.hir.get(field_id);
            let field_ty = self.lower_ty(&field.ty);

            let field_idx = self.mir.context_mut().create_field(mir_id, field_ty);

            self.field_map.insert(field_id, field_idx);
        }
    }
}