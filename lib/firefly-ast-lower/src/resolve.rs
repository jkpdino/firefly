use firefly_hir::{
    resolve::{StaticMemberTable, Symbol, SymbolTable, VisibleWithin}, ty::{HasType, Ty}, value::{HasValue, Value}, Entity, Id
};

use crate::{errors::SymbolError, AstLowerer};
use firefly_ast::Path;

impl AstLowerer {
    pub fn resolve_value(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Value> {
        let value_node = self.resolve_path(path, from, symbol_table)?;

        let Some(has_value) = self.context.try_get::<HasValue>(value_node) else {
            let symbol_name_span = self.context.try_get::<Symbol>(value_node)
                .expect("internal compiler error: doesn't have a symbol")
                .name.span;

            self.emit(SymbolError::NotAValue(path.span, symbol_name_span));
            return None;
        };

        let mut value = has_value.value.clone();
        value.span = path.span;
        return Some(value);
    }

    pub fn resolve_type(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Ty> {
        let type_node = self.resolve_path(path, from, symbol_table)?;

        let Some(has_ty) = self.context.try_get::<HasType>(type_node) else {
            let symbol_name_span = self.context.try_get::<Symbol>(type_node)
                .expect("internal compiler error: doesn't have a symbol")
                .name.span;

            self.emit(SymbolError::NotAType(path.span, symbol_name_span));
            return None;
        };

        let mut ty = has_ty.ty.clone();
        ty.span = path.span;
        return Some(ty);
    }

    pub fn resolve_path(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Id<Entity>> {
        let first_segment = path.segments.first()?;

        let Some(mut current_entity) = symbol_table.get(&first_segment.name.item) else {
            self.emit(SymbolError::NotFound(first_segment.name.clone()));

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
                let current_entity_name_span = self.context.get(current_entity).name.span;

                self.emit(SymbolError::NotFoundIn(segment.name.clone(), current_entity_name_span));

                return None;
            };

            let Some(VisibleWithin(scope)) = self.context.try_get_computed(symbol).cloned() else {
                panic!("internal compiler error: can't calculate visibility")
            };

            // todo: if it becomes a performance concern, cache ancestors
            if !self.has_ancestor(from, scope) {
                let symbol_name = self.context.get(symbol).name.span;
                self.emit(SymbolError::NotVisible(segment.name.clone(), symbol_name));
                return None;
            }

            current_entity = symbol;
        }

        return Some(current_entity.as_base());
    }

    fn has_ancestor(&self, entity: Id<Entity>, ancestor: Id<Entity>) -> bool {
        if ancestor == entity {
            return true;
        }

        let mut current = entity;
        while let Some(parent) = self.context.parent(current) {
            if ancestor == parent {
                return true;
            }

            current = parent;
        }

        return false;
    }
}
