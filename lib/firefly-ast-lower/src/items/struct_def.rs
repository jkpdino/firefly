use crate::{AstLowerer, Lower, SymbolDesc};
use firefly_ast::struct_def::StructDef as AstStruct;
use firefly_hir::{items::StructDef as HirStructDef, Entity, Id};


impl Lower for AstStruct {
    fn id(&self) -> Id<Entity> {
        self.id.as_base()
    }

    fn get_symbol(&self) -> Option<SymbolDesc> {
        let name = self.name.clone();
        let visibility = self.visibility.clone();

        Some(SymbolDesc { name, visibility, static_kw: None })
    }

    fn lower_def(&self, _: Id<Entity>, lowerer: &mut AstLowerer) {
        lowerer.context_mut().create(HirStructDef { id: self.id });
    }

    fn lower_code(&self, _: Id<Entity>, _: &mut AstLowerer) { }
}