use firefly_hir::{
    func::Callable, resolve::{InstanceMemberTable, StaticMemberTable, Symbol, SymbolTable, VisibleWithin}, ty::{HasType, Ty}, value::{HasValue, HasValueIn, Value, ValueKind}, Entity, Id
};
use firefly_span::Span;

use crate::{errors::SymbolError, AstLowerer};
use firefly_ast::{Path, PathSegment};

impl AstLowerer {
    pub fn resolve_value(&mut self, path: &Path, from: Id<Entity>, symbol_table: &SymbolTable) -> Option<Value> {
        let (value_node, member_segments) = self.resolve_path(path, from, symbol_table)?;

        let mut value =
            if let Some(has_value) = self.context.try_get::<HasValue>(value_node) {
                Value {
                    span: path.span,
                    ..has_value.value.clone()
                }
            }
            else if let Some(has_value_in) = self.context.try_get::<HasValueIn>(value_node) {
                let self_value = self.self_value.clone().unwrap(); // todo: error

                self.get_member_of(self_value, path.span, has_value_in)
            }
            else {
                let symbol_name_span = self.context.try_get::<Symbol>(value_node)
                    .expect("internal compiler error: doesn't have a symbol")
                    .name.span;

                self.emit(SymbolError::NotAValue(path.span, symbol_name_span));
                return None;
            };

        for segment in member_segments {
            let child = self.resolve_instance_member(value, segment, from)?;

            value = child;
        }
        
        return Some(value);
    }

    pub fn resolve_instance_member(&mut self, value: Value, segment: PathSegment, from: Id<Entity>) -> Option<Value> {
        let Some(instance) = value.ty.references() else {
            self.emit(SymbolError::NoMembersOf(value.clone()));
            return None;
        };

        let instance_member_table = self.context.try_get_computed::<InstanceMemberTable>(instance)
            .expect("internal compiler error: type doesn't have an instance member table");

        let Some(symbol) = instance_member_table.lookup(&segment.name.item) else {
            self.emit(SymbolError::NoMemberOn(segment.name.clone(), value.clone()));
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

        let Some(value_in) = self.context().try_get::<HasValueIn>(symbol) else {
            let symbol_name = self.context.get(symbol).name.span;

            self.emit(SymbolError::MemberNotAValue(segment.name.clone(), symbol_name));
            return None;
        };

        let span = value.span.to(segment.name.span);

        return Some(self.get_member_of(value, span, value_in))
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

    fn get_member_of(&self, value: Value, span: Span, value_in: &HasValueIn) -> Value {
        match value_in {
            HasValueIn::Field(field_id) => {
                let field = self.context.get(*field_id);

                let kind = ValueKind::FieldOf(Box::new(value), field.id);
                let mut ty = field.ty.clone();

                ty.span = span;
                

                Value { kind, ty, span }
            }

            HasValueIn::Method(method_id) => {
                let signature = self.context.try_get::<Callable>(*method_id).unwrap();

                let kind = ValueKind::InstanceFunc(Box::new(value), *method_id);

                let mut ty = signature.ty();
                ty.span = span;

                Value { kind, ty, span }
            }
        }
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
