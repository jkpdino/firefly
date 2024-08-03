use crate::{AstLowerer, Lower, SymbolDesc};
use firefly_ast::func::{Func as AstFunc, FuncParam as AstFuncParam, FuncSignature as AstFuncSignature};
use firefly_hir::{func::{Callable, Func as HirFunc, FuncParam as HirFuncParam}, resolve::{Symbol, SymbolTable}, stmt::Local, ty::{HasType, Ty, TyKind}, value::{HasSelf, HasValue, Value, ValueKind}, Entity, Id, Name, Visibility};
use firefly_span::{Span, Spanned};
use itertools::Itertools;

impl AstLowerer {
    fn lower_signature(&mut self, signature: &AstFuncSignature, parent: Id<Entity>, symbol_table: &SymbolTable) -> Callable {
        let return_ty = signature.return_ty.as_ref()
            .map(|return_ty| self.lower_ty(return_ty, parent, symbol_table))
            .unwrap_or_else(|| Ty::new_unspanned(TyKind::Unit));
        let params = signature.params.iter()
            .map(|param| self.lower_func_parameter(param, parent, symbol_table))
            .collect_vec();

        let parent_of_parent = self.context.parent(parent).unwrap();

        if let Some(has_type) = self.context.try_get::<HasType>(parent_of_parent) {
            let receiver = has_type.ty.clone();

            return Callable { params, return_ty, receiver: Some(receiver) }
        }

        Callable { params, return_ty, receiver: None }
    }

    fn lower_func_parameter(&mut self, param: &Spanned<AstFuncParam>, parent: Id<Entity>, symbol_table: &SymbolTable) -> HirFuncParam {
        let ty = self.lower_ty(&param.item.ty, parent, symbol_table);
        let bind_name = self.lower_name(&param.item.name);

        self.create_local(parent, &bind_name, &ty);

        HirFuncParam { ty, bind_name }
    }

    pub fn create_local(&mut self, parent: Id<Entity>, name: &Name, ty: &Ty) -> Id<Local> {
        let local = Id::default();
        
        self.context.create_with_parent(parent, (
            Local {
                id: local,
                ty: ty.clone(),
            },
            Symbol {
                name: name.clone(),
                visibility: Visibility::Local,
                is_static: true
            },
            HasValue {
                value: Value::new(ValueKind::Local(local), ty.clone(), Default::default()),
            }
        ))
    }
}

impl Lower for AstFunc {
    fn id(&self) -> Id<Entity> {
        return self.id.as_base();
    }

    fn get_symbol(&self) -> Option<SymbolDesc> {
        let name = self.name.clone();
        let visibility = self.visibility.clone();
        let static_kw = self.static_kw;

        Some(SymbolDesc { name, visibility, static_kw })
    }

    fn lower_def(&self, parent: Id<Entity>, lowerer: &mut AstLowerer) {
        let Some(symbol_table) = lowerer.context_mut().try_get_computed::<SymbolTable>(parent).cloned() else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let signature = lowerer.lower_signature(&self.signature, self.id.as_base(), &symbol_table);
        let ty = signature.ty();

        if let Some(receiver) = &signature.receiver {
            let self_id = lowerer.create_local(self.id(), &Name::internal("self"), &receiver);

            lowerer.context_mut().create((
                HirFunc { id: self.id },
                HasSelf { local: self_id, ty: receiver.clone() },
                signature,
            ));
        }
        else {
            let value = Value::new(ValueKind::StaticFunc(self.id), ty, Span::default());

            lowerer.context_mut().create((
                HirFunc { id: self.id },
                HasValue { value },
                signature
            ));
        }
    }
    
    fn lower_code(&self, _: Id<Entity>, lowerer: &mut AstLowerer) {
        let old_self_value =
        if let Some(HasSelf { local, ty }) = lowerer.context().try_get(self.id) {
            let self_value = Value::new(
                ValueKind::Local(*local),
                ty.clone(),
                Span::default()
            );

            lowerer.self_value.replace(self_value)
        }
        else { None };

        let mut code_symbol_table = lowerer.context_mut().try_get_computed::<SymbolTable>(self.id).cloned()
            .expect("internal compiler error: function is not a namespace");

        lowerer.lower_code_block(&self.body, self.id.as_base(), &mut code_symbol_table);

        if let Some(old_self_value) = old_self_value {
            lowerer.self_value.replace(old_self_value);
        }
    }
}