use firefly_hir::{
    func::Callable,
    resolve::{
        InstanceMemberTable, StaticMemberTable, Symbol, SymbolCollection, SymbolTable,
        VisibleWithin,
    },
    ty::{HasType, Ty, TyKind},
    value::{HasValue, HasValueIn, Value, ValueKind},
    Entity, HirContext, Id,
};
use firefly_span::Span;

use crate::{errors::SymbolError, AstLowerer};
use firefly_ast::{
    operator::{InfixOperator, PrefixOperator},
    Path, PathSegment,
};

impl AstLowerer {
    pub fn resolve_value(
        &mut self,
        path: &Path,
        from: Id<Entity>,
        symbol_table: &SymbolTable,
    ) -> Option<Value> {
        self.resolve_value_with(path, from, symbol_table, |_, _| true)
    }

    pub fn resolve_value_with(
        &mut self,
        path: &Path,
        from: Id<Entity>,
        symbol_table: &SymbolTable,
        predicate: impl Fn(Id<Symbol>, &HirContext) -> bool,
    ) -> Option<Value> {
        let (symbol_collection, member_segments) = self.resolve_path(path, from, symbol_table)?;

        let filtered_symbols =
            symbol_collection.symbols_matching(|id| predicate(id, &self.context));

        if let Some(value_node) = filtered_symbols.single() {
            let mut value = if let Some(has_value) = self.context.try_get::<HasValue>(value_node) {
                Value {
                    span: path.span,
                    ..has_value.value.clone()
                }
            } else if let Some(has_value_in) = self.context.try_get::<HasValueIn>(value_node) {
                let self_value = self.self_value.clone().unwrap(); // todo: error

                self.get_member_of(self_value, path.span, has_value_in)
            } else {
                let symbol_name_span = self
                    .context
                    .try_get::<Symbol>(value_node)
                    .expect("internal compiler error: doesn't have a symbol")
                    .name
                    .span;

                self.emit(SymbolError::NotAValue(path.span, symbol_name_span));
                return None;
            };

            for segment in member_segments {
                let child = self.resolve_instance_member(value, segment, from)?;

                value = child;
            }

            return Some(value);
        } else if filtered_symbols.is_empty() {
            // todo: throw a no matching value error
        } else {
            // todo: throw an ambiguous value error
        }

        return None;
    }

    pub fn get_integer_prefix_operator(
        &mut self,
        operator: &PrefixOperator,
        value: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            PrefixOperator::Identity => ("identity", TyKind::Integer),
            PrefixOperator::Invert => ("bitnot", TyKind::Integer),
            PrefixOperator::Negate => ("negate", TyKind::Integer),
        };

        let op_func_kind = TyKind::Func(
            vec![Ty::new_unspanned(TyKind::Integer)],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![value],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }

    pub fn get_float_prefix_operator(
        &mut self,
        operator: &PrefixOperator,
        value: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            PrefixOperator::Identity => ("identity_float", TyKind::Float),
            PrefixOperator::Invert => return None,
            PrefixOperator::Negate => ("identity_negate", TyKind::Float),
        };

        let op_func_kind = TyKind::Func(
            vec![Ty::new_unspanned(TyKind::Float)],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![value],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }

    pub fn get_boolean_prefix_operator(
        &mut self,
        operator: &PrefixOperator,
        value: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            PrefixOperator::Identity => return None,
            PrefixOperator::Invert => ("not", TyKind::Integer),
            PrefixOperator::Negate => return None,
        };

        let op_func_kind = TyKind::Func(
            vec![Ty::new_unspanned(TyKind::Integer)],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![value],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }

    pub fn get_integer_operator(
        &mut self,
        operator: &InfixOperator,
        left: Value,
        right: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            InfixOperator::Add => ("add", TyKind::Integer),
            InfixOperator::Subtract => ("sub", TyKind::Integer),
            InfixOperator::Multiply => ("mul", TyKind::Integer),
            InfixOperator::Divide => ("div", TyKind::Integer),
            InfixOperator::Modulo => ("rem", TyKind::Integer),
            InfixOperator::ShiftLeft => ("left_shift", TyKind::Integer),
            InfixOperator::ShiftRight => ("right_shift", TyKind::Integer),
            InfixOperator::BitAnd => ("bitand", TyKind::Integer),
            InfixOperator::BitXor => ("bitxor", TyKind::Integer),
            InfixOperator::BitOr => ("bitor", TyKind::Integer),
            InfixOperator::CompareLessThan => ("lt_int", TyKind::Bool),
            InfixOperator::CompareGreaterThan => ("gt_int", TyKind::Bool),
            InfixOperator::CompareLessThanOrEqual => ("leq_int", TyKind::Bool),
            InfixOperator::CompareGreaterThanOrEqual => ("geq_int", TyKind::Bool),
            InfixOperator::CompareEqual => ("eq_int", TyKind::Bool),
            InfixOperator::CompareNotEqual => ("neq_int", TyKind::Bool),
            InfixOperator::LogicalAnd => return None,
            InfixOperator::LogicalOr => return None,
            InfixOperator::AddAssign => return None,
            InfixOperator::SubtractAssign => return None,
            InfixOperator::MultiplyAssign => return None,
            InfixOperator::DivideAssign => return None,
            InfixOperator::ModuloAssign => return None,
            InfixOperator::ShiftLeftAssign => return None,
            InfixOperator::ShiftRightAssign => return None,
            InfixOperator::BitAndAssign => return None,
            InfixOperator::BitOrAssign => return None,
            InfixOperator::BitXorAssign => return None,
            InfixOperator::Assign => return None,
        };

        let op_func_kind = TyKind::Func(
            vec![
                Ty::new_unspanned(TyKind::Integer),
                Ty::new_unspanned(TyKind::Integer),
            ],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![left, right],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }

    pub fn get_float_operator(
        &mut self,
        operator: &InfixOperator,
        left: Value,
        right: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            InfixOperator::Add => ("fadd", TyKind::Integer),
            InfixOperator::Subtract => ("fsub", TyKind::Integer),
            InfixOperator::Multiply => ("fmul", TyKind::Integer),
            InfixOperator::Divide => ("fdiv", TyKind::Integer),
            InfixOperator::Modulo => ("frem", TyKind::Integer),
            InfixOperator::ShiftLeft
            | InfixOperator::ShiftRight
            | InfixOperator::BitAnd
            | InfixOperator::BitXor
            | InfixOperator::BitOr => return None,
            InfixOperator::CompareLessThan => ("lt_float", TyKind::Bool),
            InfixOperator::CompareGreaterThan => ("gt_float", TyKind::Bool),
            InfixOperator::CompareLessThanOrEqual => ("leq_float", TyKind::Bool),
            InfixOperator::CompareGreaterThanOrEqual => ("geq_float", TyKind::Bool),
            InfixOperator::CompareEqual => ("eq_float", TyKind::Bool),
            InfixOperator::CompareNotEqual => ("neq_float", TyKind::Bool),
            InfixOperator::LogicalAnd => return None,
            InfixOperator::LogicalOr => return None,
            InfixOperator::AddAssign => return None,
            InfixOperator::SubtractAssign => return None,
            InfixOperator::MultiplyAssign => return None,
            InfixOperator::DivideAssign => return None,
            InfixOperator::ModuloAssign => return None,
            InfixOperator::ShiftLeftAssign => return None,
            InfixOperator::ShiftRightAssign => return None,
            InfixOperator::BitAndAssign => return None,
            InfixOperator::BitOrAssign => return None,
            InfixOperator::BitXorAssign => return None,
            InfixOperator::Assign => return None,
        };

        let op_func_kind = TyKind::Func(
            vec![
                Ty::new_unspanned(TyKind::Float),
                Ty::new_unspanned(TyKind::Float),
            ],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![left, right],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }

    pub fn get_boolean_operator(
        &mut self,
        operator: &InfixOperator,
        left: Value,
        right: Value,
        span: Span,
    ) -> Option<Value> {
        let (builtin_name, return_type_kind) = match operator {
            InfixOperator::Add
            | InfixOperator::Subtract
            | InfixOperator::Multiply
            | InfixOperator::Divide
            | InfixOperator::Modulo
            | InfixOperator::ShiftLeft
            | InfixOperator::ShiftRight
            | InfixOperator::BitAnd
            | InfixOperator::BitXor
            | InfixOperator::BitOr
            | InfixOperator::CompareLessThan
            | InfixOperator::CompareGreaterThan
            | InfixOperator::CompareLessThanOrEqual
            | InfixOperator::CompareGreaterThanOrEqual => return None,
            InfixOperator::CompareEqual => ("eq_bool", TyKind::Bool),
            InfixOperator::CompareNotEqual => ("neq_bool", TyKind::Bool),
            InfixOperator::LogicalAnd => ("and", TyKind::Bool),
            InfixOperator::LogicalOr => ("or", TyKind::Bool),
            InfixOperator::AddAssign
            | InfixOperator::SubtractAssign
            | InfixOperator::MultiplyAssign
            | InfixOperator::DivideAssign
            | InfixOperator::ModuloAssign
            | InfixOperator::ShiftLeftAssign
            | InfixOperator::ShiftRightAssign
            | InfixOperator::BitAndAssign
            | InfixOperator::BitOrAssign
            | InfixOperator::BitXorAssign
            | InfixOperator::Assign => return None,
        };

        let op_func_kind = TyKind::Func(
            vec![
                Ty::new_unspanned(TyKind::Bool),
                Ty::new_unspanned(TyKind::Bool),
            ],
            Box::new(Ty::new_unspanned(return_type_kind.clone())),
        );

        return Some(Value::new(
            ValueKind::Invoke(
                Box::new(Value::new(
                    ValueKind::BuiltinFunc(builtin_name),
                    Ty::new(op_func_kind, span),
                    span,
                )),
                vec![left, right],
            ),
            Ty::new(return_type_kind, span),
            span,
        ));
    }
    pub fn resolve_instance_member(
        &mut self,
        value: Value,
        segment: PathSegment,
        from: Id<Entity>,
    ) -> Option<Value> {
        let Some(instance) = value.ty.defined_by() else {
            self.emit(SymbolError::NoMembersOf(value.clone()));
            return None;
        };

        let instance_member_table = self
            .context
            .try_get_computed::<InstanceMemberTable>(instance)
            .expect("internal compiler error: type doesn't have an instance member table");

        let Some(symbol_collection) = instance_member_table.lookup(&segment.name.item) else {
            self.emit(SymbolError::NoMemberOn(segment.name.clone(), value.clone()));
            return None;
        };

        // Handle the single case
        if let Some(symbol) = symbol_collection.single() {
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

                self.emit(SymbolError::MemberNotAValue(
                    segment.name.clone(),
                    symbol_name,
                ));
                return None;
            };

            let span = value.span.to(segment.name.span);

            return Some(self.get_member_of(value, span, value_in));
        }

        // todo: Handle the multiple case
        // We have to filter the symbols by the context
        // For example, if we have call arguments, we should only return matching functions
        return None;
    }

    pub fn resolve_type(
        &mut self,
        path: &Path,
        from: Id<Entity>,
        symbol_table: &SymbolTable,
    ) -> Option<Ty> {
        let (symbol_collection, member_segments) = self.resolve_path(path, from, symbol_table)?;

        // filter the symbols to only include types
        let type_symbols =
            symbol_collection.symbols_matching(|id| self.context().has::<HasType>(id));

        // todo: handle member_segments
        if !member_segments.is_empty() {
            let non_type_symbol = symbol_collection
                .symbols
                .first()
                .expect("internal compiler error: doesn't have a symbol");

            let current_symbol = self
                .context
                .try_get::<Symbol>(*non_type_symbol)
                .expect("internal compiler error: doesn't have a symbol");
            let symbol_name_span = current_symbol.name.span;

            self.emit(SymbolError::NotFoundIn(
                member_segments[0].name.clone(),
                symbol_name_span,
            ));
        }

        if let Some(type_node) = type_symbols.single() {
            let has_type = self
                .context
                .try_get::<HasType>(type_node)
                .expect("internal compiler error: doesn't have a type");

            let mut ty = has_type.ty.clone();
            ty.span = path.span;
            return Some(ty);
        }
        // handle the zero case
        else if type_symbols.is_empty() {
            let non_type_symbol = symbol_collection
                .single()
                .expect("internal compiler error: doesn't have a symbol");

            let symbol_name_span = self
                .context
                .try_get::<Symbol>(non_type_symbol)
                .expect("internal compiler error: doesn't have a symbol")
                .name
                .span;

            self.emit(SymbolError::NotAType(path.span, symbol_name_span));
            return None;
        }
        // handle the ambiguous case
        else {
            // todo: throw an ambiguous type error
            todo!();
        }
    }

    /// Resolves a path to a symbol collection and any remaining path segments.
    ///
    /// This function takes a path (like `a.b.c`), a starting entity, and a symbol table,
    /// and attempts to resolve as much of the path as possible. It returns:
    /// - The collection of symbols that match the resolved part of the path
    /// - Any remaining path segments that couldn't be resolved (for member access)
    ///
    /// The resolution process works by:
    /// 1. Looking up the first segment in the symbol table
    /// 2. For each subsequent segment, looking it up in the static member table of the previous entity
    /// 3. Checking visibility of each symbol to ensure it's accessible from the 'from' entity
    ///
    /// If resolution fails at any point, appropriate errors are emitted.
    pub fn resolve_path(
        &mut self,
        path: &Path,
        from: Id<Entity>,
        symbol_table: &SymbolTable,
    ) -> Option<(SymbolCollection, Vec<PathSegment>)> {
        // Get the first segment of the path
        let first_segment = path.segments.first()?;

        // Get the symbol for the first segment from the symbol table
        let Some(mut current_entity) = symbol_table.get(&first_segment.name.item).cloned() else {
            // Emit an error if the first segment can't be found in the symbol table
            self.emit(SymbolError::NotFound(first_segment.name.clone()));
            return None;
        };

        // Process each subsequent segment in the path
        for (i, segment) in path.segments.iter().enumerate().skip(1) {
            // We need a single entity to look up members in
            let Some(single_entity) = current_entity.single() else {
                break;
            };

            // Get the static member table for the current entity
            let Some(static_member_table) = self
                .context
                .try_get_computed::<StaticMemberTable>(single_entity)
            else {
                todo!();
            };

            // Look up the current segment in the static member table
            let Some(symbol) = static_member_table.lookup(&segment.name.item) else {
                // If the segment isn't found, return the current entity and the remaining segments
                // This allows for partial resolution where the rest might be instance members
                return Some((
                    SymbolCollection::new_single(single_entity),
                    path.segments[i..].to_vec(),
                ));
            };

            // We need a single symbol to continue resolution
            let Some(single_symbol) = symbol.single() else {
                // Handle ambiguous symbols
                todo!();
            };

            // Check if the symbol is visible from the 'from' entity
            let Some(VisibleWithin(scope)) = self.context.try_get_computed(single_symbol).cloned()
            else {
                panic!("internal compiler error: can't calculate visibility")
            };

            // Check if 'from' is within the visibility scope of the symbol
            // todo: if it becomes a performance concern, cache ancestors
            if !self.has_ancestor(from, scope) {
                let symbol_name = self.context.get(single_symbol).name.span;
                self.emit(SymbolError::NotVisible(segment.name.clone(), symbol_name));
                return None;
            }

            // Update current_entity to continue resolution
            current_entity = symbol;
        }

        // Convert the final symbol collection to the expected return format
        // Each symbol is returned with an empty list of remaining segments
        return Some((current_entity, vec![]));
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
