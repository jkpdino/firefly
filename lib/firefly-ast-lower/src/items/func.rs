use crate::AstLowerer;
use firefly_ast::func::{Func as AstFunc, FuncParam as AstFuncParam, FuncSignature as AstFuncSignature};
use firefly_hir::{func::{Callable, Func as HirFunc, FuncParam as HirFuncParam}, resolve::{Symbol, SymbolTable}, stmt::Local, ty::{Ty, TyKind}, value::{HasValue, Value, ValueKind}, Entity, Id, Name, Visibility};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_func(&mut self, func: &AstFunc, parent: Id<Entity>) {
        let Some(symbol_table) = self.context.try_get_computed::<SymbolTable>(parent).cloned() else {
            panic!("internal compiler error: parent is not a namespace")
        };        

        let signature = self.lower_signature(&func.signature, func.id.as_base(), &symbol_table);

        let func_entity = HirFunc { id: func.id };
        self.context.create((func_entity, signature));

        let mut code_symbol_table = self.context.try_get_computed::<SymbolTable>(func.id).cloned()
            .expect("internal compiler error: function is not a namespace");

        self.lower_code_block(&func.body, func.id.as_base(), &mut code_symbol_table);
    }

    fn lower_signature(&mut self, signature: &AstFuncSignature, parent: Id<Entity>, symbol_table: &SymbolTable) -> Callable {
        let return_ty = signature.return_ty.as_ref()
            .map(|return_ty| self.lower_ty(return_ty, parent, symbol_table))
            .unwrap_or_else(|| Ty::new_unspanned(TyKind::Unit));
        let params = signature.params.iter()
            .map(|param| self.lower_func_parameter(param, parent, symbol_table))
            .collect_vec();

        Callable { params, return_ty }
    }

    fn lower_func_parameter(&mut self, param: &Spanned<AstFuncParam>, parent: Id<Entity>, symbol_table: &SymbolTable) -> HirFuncParam {
        let ty = self.lower_ty(&param.item.ty, parent, symbol_table);
        let bind_name = self.lower_name(&param.item.name);

        self.create_local(parent, &bind_name, &ty);

        HirFuncParam { ty, bind_name }
    }

    pub fn create_local(&mut self, parent: Id<Entity>, name: &Name, ty: &Ty) {
        let local = Id::default();
        
        self.context.create_with_parent(parent, (
            Local {
                id: local,
                ty: ty.clone(),
            },
            Symbol {
                name: name.clone(),
                visibility: Visibility::Local,
            },
            HasValue {
                value: Value::new(ValueKind::Local(local), ty.clone(), Default::default()),
            }
        ));
    }
}
