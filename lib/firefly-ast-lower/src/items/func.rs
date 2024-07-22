use crate::AstLowerer;
use firefly_ast::func::Func as AstFunc;
use firefly_hir::{
    func::Func as HirFunc,
    resolve::{Namespace, SymbolTable},
    Entity, Id,
};

impl AstLowerer {
    pub fn lower_func(&mut self, func: &AstFunc, parent: Id<Entity>) {
        let Some(namespace_id) = self.context.cast_id(parent) else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let symbol_table = SymbolTable::get_for_namespace(namespace_id, &self.context);

        let func_entity = HirFunc { id: func.id };

        self.context.create(func_entity);
    }
}
