use crate::{AstLowerer, Lower, SymbolDesc};
use firefly_ast::init::Init as AstInit;
use firefly_hir::{
    func::Init as HirInit,
    resolve::{delegate::StaticDelegate, SymbolTable},
    ty::TyKind,
    value::{HasSelf, HasValue, Value, ValueKind},
    Entity, Id, Name,
};
use firefly_span::{Span, Spanned};

impl Lower for AstInit {
    fn id(&self) -> Id<Entity> {
        return self.id.as_base();
    }

    fn get_symbol(&self) -> Option<SymbolDesc> {
        let visibility = self.visibility.clone();

        Some(SymbolDesc {
            name: Spanned::new("init".into(), Span::DUMMY),
            visibility,
            static_kw: None,
        })
    }

    fn lower_def(&self, parent: Id<Entity>, lowerer: &mut AstLowerer) {
        let Some(symbol_table) = lowerer
            .context_mut()
            .try_get_computed::<SymbolTable>(parent)
            .cloned()
        else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let signature = lowerer.lower_signature(&self.signature, self.id.as_base(), &symbol_table);

        let Some(receiver) = &signature.receiver else {
            panic!("internal compiler error: receiver is not a namespace")
        };
        let self_id = lowerer.create_local(self.id(), &Name::internal("self"), &receiver);

        // create the signature type
        let mut ty = signature.ty();
        if let TyKind::Func(_, return_ty) = &mut ty.kind {
            return_ty.kind = receiver.kind.clone();
            return_ty.id = receiver.id;
        }

        // create the ctor value
        let value = Value::new(ValueKind::Init(receiver.clone(), self.id), ty, Span::DUMMY);

        lowerer.context_mut().create((
            HirInit { id: self.id },
            HasValue { value },
            HasSelf {
                local: self_id,
                ty: receiver.clone(),
            },
            StaticDelegate,
            signature,
        ));
    }

    fn lower_code(&self, _: Id<Entity>, lowerer: &mut AstLowerer) {
        let old_self_value = if let Some(HasSelf { local, ty }) = lowerer.context().try_get(self.id)
        {
            let self_value = Value::new(ValueKind::Local(*local), ty.clone(), Span::default());

            lowerer.self_value.replace(self_value)
        } else {
            None
        };

        let mut code_symbol_table = lowerer
            .context_mut()
            .try_get_computed::<SymbolTable>(self.id)
            .cloned()
            .expect("internal compiler error: function is not a namespace");

        lowerer.lower_code_block(&self.body, self.id.as_base(), &mut code_symbol_table);

        if let Some(old_self_value) = old_self_value {
            lowerer.self_value.replace(old_self_value);
        }
    }
}
