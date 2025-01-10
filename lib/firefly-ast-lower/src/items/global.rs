use firefly_ast::struct_def::Field;
use firefly_hir::{
    items::{Field as HirField, Global, SourceFile},
    resolve::SymbolTable,
    value::{HasValue, HasValueIn, Value, ValueKind},
    Entity, Id,
};

use crate::{errors::DeclarationError, AstLowerer, Lower, SymbolDesc};

impl Lower for Field {
    fn id(&self) -> Id<Entity> {
        return self.id;
    }

    fn get_symbol(&self) -> Option<SymbolDesc> {
        let name = self.name.clone();
        let visibility = self.visibility.clone();
        let static_kw = self.static_kw;

        Some(SymbolDesc {
            name,
            visibility,
            static_kw,
        })
    }

    fn lower_def(&self, parent: Id<Entity>, lowerer: &mut AstLowerer) {
        let is_static = lowerer.context().has::<SourceFile>(parent) || self.static_kw.is_some();

        let Some(symbol_table) = lowerer
            .context_mut()
            .try_get_computed::<SymbolTable>(parent)
            .cloned()
        else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let ty = lowerer.lower_ty(&self.ty, parent, &symbol_table);

        if is_static {
            let id = unsafe { self.id.cast::<Global>() };

            let value = Value::new(ValueKind::Global(id), ty.clone(), self.name.span);
            lowerer.context_mut().add_component(id, HasValue { value });
        } else {
            let id = unsafe { self.id.cast::<HirField>() };

            lowerer.context_mut().create(HirField { id, ty });
            lowerer
                .context_mut()
                .add_component(id, HasValueIn::Field(id));
        }
    }

    fn lower_code(&self, parent: Id<Entity>, lowerer: &mut AstLowerer) {
        let is_static = lowerer.context().has::<SourceFile>(parent) || self.static_kw.is_some();

        if !is_static {
            return;
        }

        let id = unsafe { self.id.cast::<Global>() };

        let Some(mut symbol_table) = lowerer
            .context_mut()
            .try_get_computed::<SymbolTable>(parent)
            .cloned()
        else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let Some(default) = &self.default else {
            lowerer.emit(DeclarationError::GlobalVarNoDefault(self.name.clone()));
            return;
        };

        let ty = lowerer.lower_ty(&self.ty, parent, &symbol_table);
        let default_value =
            lowerer.lower_value(&default, parent, &mut symbol_table, Default::default());

        lowerer.context_mut().create(Global {
            id,
            ty,
            default_value,
        });
    }
}
