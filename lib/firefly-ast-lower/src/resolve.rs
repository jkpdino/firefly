use firefly_hir::{
    items::Field, resolve::{InstanceMemberTable, StaticMemberTable, Symbol, SymbolTable, VisibleWithin}, ty::{HasType, Ty}, value::{HasValue, Value, ValueKind}, Entity, Id
};

use crate::{errors::SymbolError, AstLowerer};
use firefly_ast::{Path, PathSegment};

impl AstLowerer {
    pub fn resolve_value(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Value> {
        let (value_node, member_segments) = self.resolve_path(path, from, symbol_table)?;

        let Some(has_value) = self.context.try_get::<HasValue>(value_node) else {
            let symbol_name_span = self.context.try_get::<Symbol>(value_node)
                .expect("internal compiler error: doesn't have a symbol")
                .name.span;

            self.emit(SymbolError::NotAValue(path.span, symbol_name_span));
            return None;
        };

        let mut value = has_value.value.clone();
        value.span = path.span;

        for segment in member_segments {
            let child = self.resolve_instance_member(value, segment, from)?;

            value = child;
        }
        
        return Some(value);
    }

    fn resolve_instance_member(&mut self, value: Value, segment: PathSegment, from: Id<Entity>) -> Option<Value> {
        let Some(instance) = value.ty.references() else {
            // todo: error
            println!("error: not a reference type");
            return None;
        };

        let instance_member_table = self.context.try_get_computed::<InstanceMemberTable>(instance)
            .expect("internal compiler error: type doesn't have an instance member table");

        let Some(symbol) = instance_member_table.lookup(&segment.name.item) else {
            println!("error: member not found: {:?}", segment.name.item);
            return None;
        };

        let Some(VisibleWithin(scope)) = self.context.try_get_computed(symbol).cloned() else {
            panic!("internal compiler error: can't calculate visibility")
        };

        if !self.has_ancestor(from, scope) {
            let symbol_name = self.context.get(symbol).name.span;
            self.emit(SymbolError::NotVisible(segment.name.clone(), symbol_name));
            return None;
        }

        if let Some(field) = self.context().try_get::<Field>(symbol) {
            let kind = ValueKind::FieldOf(Box::new(value), field.id);
            let mut ty = field.ty.clone();
            let span = segment.name.span;

            ty.span = span;
            

            Some(Value { kind, ty, span })
        }
        else {
            return None
        }
    }

    pub fn resolve_type(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Ty> {
        let (type_node, member_segments) = self.resolve_path(path, from, symbol_table)?;

        // todo: handle member_segments
        if !member_segments.is_empty() {
            let current_symbol = self.context.try_get::<Symbol>(type_node).expect("internal compiler error: doesn't have a symbol");
            let symbol_name_span = current_symbol.name.span;

            self.emit(SymbolError::NotFoundIn(member_segments[0].name.clone(), symbol_name_span));
        }

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

    pub fn resolve_path(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<(Id<Entity>, Vec<PathSegment>)> {
        let first_segment = path.segments.first()?;

        let Some(mut current_entity) = symbol_table.get(&first_segment.name.item) else {
            self.emit(SymbolError::NotFound(first_segment.name.clone()));

            return None;
        };

        for (i, segment) in path.segments.iter().enumerate().skip(1) {
            let Some(static_member_table) = self
                .context
                .try_get_computed::<StaticMemberTable>(current_entity)
            else {
                todo!();
            };

            let Some(symbol) = static_member_table.lookup(&segment.name.item) else {
                return Some((current_entity.as_base(), path.segments[i..].to_vec()));
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

        return Some((current_entity.as_base(), vec![]));
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
