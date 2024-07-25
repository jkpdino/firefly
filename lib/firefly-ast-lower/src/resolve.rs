use firefly_hir::{
    resolve::{StaticMemberTable, SymbolTable}, ty::{HasType, Ty}, value::{HasValue, Value}, Entity, Id
};

use crate::AstLowerer;
use firefly_ast::Path;

impl AstLowerer {
    pub fn resolve_value(&mut self, path: &Path, symbol_table: &SymbolTable) -> Option<Value> {
        let value_node = self.resolve_path(path, symbol_table)?;

        if let Some(has_value) = self.context.try_get::<HasValue>(value_node) {
            let mut value = has_value.value.clone();
            value.span = path.span;
            return Some(value);
        } else {
            println!("error: not a value");
            return None;
        }
    }

    pub fn resolve_type(&mut self, path: &Path, symbol_table: &SymbolTable) -> Option<Ty> {
        let type_node = self.resolve_path(path, symbol_table)?;

        if let Some(has_ty) = self.context.try_get::<HasType>(type_node) {
            let mut ty = has_ty.ty.clone();
            ty.span = path.span;
            return Some(ty);
        } else {
            println!("error: not a type");
            return None;
        }
    }

    pub fn resolve_path(&mut self, path: &Path, symbol_table: &SymbolTable) -> Option<Id<Entity>> {
        let first_segment = path.segments.first()?;

        let Some(mut current_entity) = symbol_table.get(&first_segment.name.item) else {
            println!(
                "can't find symbol {} in the current scope",
                first_segment.name.item
            );
            return None;
        };

        for segment in path.segments.iter().skip(1) {
            let Some(static_member_table) = self
                .context
                .try_get_computed::<StaticMemberTable>(current_entity)
            else {
                todo!();
            };

            let Some(symbol) = static_member_table.lookup(&segment.name.item) else {
                panic!("error!");
            };

            current_entity = symbol;
        }

        return Some(current_entity.as_base());
    }
}
