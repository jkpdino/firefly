use crate::AstLowerer;
use firefly_ast::func::{Func as AstFunc, FuncParam as AstFuncParam, FuncSignature as AstFuncSignature};
use firefly_hir::{func::{Callable, Func as HirFunc, FuncParam as HirFuncParam}, resolve::{Symbol, SymbolTable}, stmt::Binding, ty::Ty, value::{HasValue, Value, ValueKind}, Entity, Id, Name, Visibility};
use firefly_span::Spanned;
use itertools::Itertools;

impl AstLowerer {
    pub fn lower_func(&mut self, func: &AstFunc, parent: Id<Entity>) {
        let Some(symbol_table) = self.context.try_get_computed::<SymbolTable>(parent).cloned() else {
            panic!("internal compiler error: parent is not a namespace")
        };        

        let signature = self.lower_signature(&func.signature, func.id.as_base(), &symbol_table);

        let func_entity = HirFunc { id: func.id };
        self.context.create(func_entity);
        self.context.add_component(func.id, signature);

        let mut code_symbol_table = self.context.try_get_computed::<SymbolTable>(func.id).cloned()
            .expect("internal compiler error: function is not a namespace");

        self.lower_code_block(&func.body, parent, &mut code_symbol_table);
    }

    fn lower_signature(&mut self, signature: &AstFuncSignature, parent: Id<Entity>, symbol_table: &SymbolTable) -> Callable {
        let return_ty = self.lower_ty(&signature.return_ty, symbol_table);
        let params = signature.params.iter()
            .map(|param| self.lower_func_parameter(param, parent, symbol_table))
            .collect_vec();

        Callable { params, return_ty }
    }

    fn lower_func_parameter(&mut self, param: &Spanned<AstFuncParam>, parent: Id<Entity>, symbol_table: &SymbolTable) -> HirFuncParam {
        let ty = self.lower_ty(&param.item.ty, symbol_table);
        let bind_name = self.lower_name(&param.item.name);

        self.create_binding(parent, &bind_name, &ty);

        HirFuncParam { ty, bind_name }
    }

    pub fn create_binding(&mut self, parent: Id<Entity>, name: &Name, ty: &Ty) {
        let binding = self.context.create(Binding {
            id: Default::default(),
            ty: ty.clone(),
        });
        self.context.add_component(binding, Symbol {
            name: name.clone(),
            visibility: Visibility::Local,
        });
        self.context.add_component(binding, HasValue {
            value: Value::new(ValueKind::Local(binding), ty.clone(), Default::default()),
        });
        self.context.link(parent, binding);
    }
}
