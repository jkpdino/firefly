use crate::AstLowerer;
use firefly_ast::func::{Func as AstFunc, FuncParam as AstFuncParam, FuncSignature as AstFuncSignature};
use firefly_hir::{func::{Callable, FuncParam as HirFuncParam, Func as HirFunc}, resolve::SymbolTable, Entity, Id};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_func(&mut self, func: &AstFunc, parent: Id<Entity>) {
        let Some(symbol_table) = self.context.try_get_computed::<SymbolTable>(parent).cloned() else {
            panic!("internal compiler error: parent is not a namespace")
        };        

        let signature = self.lower_signature(&func.signature, &symbol_table);

        let func_entity = HirFunc { id: func.id };
        self.context.create(func_entity);
        self.context.add_component(func.id, signature);
    }

    fn lower_signature(&mut self, signature: &AstFuncSignature, symbol_table: &SymbolTable) -> Callable {
        let return_ty = self.lower_ty(&signature.return_ty, symbol_table);
        let params = signature.params.iter()
            .map(|param| self.lower_func_parameter(param, symbol_table))
            .collect_vec();

        Callable { params, return_ty }
    }

    fn lower_func_parameter(&mut self, param: &Spanned<AstFuncParam>, symbol_table: &SymbolTable) -> HirFuncParam {
        let ty = self.lower_ty(&param.item.ty, symbol_table);
        let bind_name = self.lower_name(&param.item.name);

        HirFuncParam { ty, bind_name }
    }
}
