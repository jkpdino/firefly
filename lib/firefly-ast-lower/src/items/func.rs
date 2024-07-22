use crate::AstLowerer;
use firefly_ast::func::Func as AstFunc;
use firefly_hir::{func::Func as HirFunc, resolve::SymbolTable, Entity, Id};

impl AstLowerer {
    pub fn lower_func(&mut self, func: &AstFunc, parent: Id<Entity>) {
        let Some(symbol_table) = self.context.try_get_computed::<SymbolTable>(parent) else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let return_ty = self.lower_ty(&func.return_ty);

        let func_entity = HirFunc { id: func.id };
        self.context.create(func_entity);
    }
}
