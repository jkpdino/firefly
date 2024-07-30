use crate::AstLowerer;
use firefly_ast::struct_def::StructDef as AstStruct;
use firefly_hir::items::StructDef as HirStructDef;


impl AstLowerer {
    pub fn lower_struct(&mut self, struct_def: &AstStruct) {
        let id = struct_def.id;

        self.context.create(HirStructDef { id });
    }
}