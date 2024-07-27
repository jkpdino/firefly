use firefly_ast::struct_def::Field;
use firefly_hir::{items::Global, resolve::SymbolTable, value::{HasValue, Value, ValueKind}, Entity, Id};

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_global(&mut self, field: &Field, parent: Id<Entity>) {
        let id = unsafe { field.id.cast::<Global>() };

        let Some(symbol_table) = self.context.try_get_computed::<SymbolTable>(parent).cloned() else {
            panic!("internal compiler error: parent is not a namespace")
        };

        let Some(default) = &field.default else {
            println!("error: global does not have a default value");
            return;
        };

        let ty = self.lower_ty(&field.ty, parent, &symbol_table);
        let default_value = self.lower_value(&default, parent, &symbol_table);

        let value = Value::new(ValueKind::Global(id), ty.clone(), field.name.span);

        self.context.create((
            Global { id, ty, default_value },
            HasValue { value }
        ));
    }
}