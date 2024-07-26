use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

use crate::{ComputedComponent, Entity, HirContext, Id};

use super::{Import, Namespace, Symbol, VisibleWithin};

/// Stores a delta so that scopes can quickly be restored
#[derive(Clone, Debug)]
struct Scope {
    old_symbols: HashMap<String, Option<Id<Symbol>>>,
}

/// Associates symbols with names, provides for
/// quick lookup
#[derive(Clone, Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, Id<Symbol>>,
    scopes: Vec<Scope>,
}

impl SymbolTable {
    /// Pushes a new scope onto the symbol table
    pub fn push_scope(&mut self) {
        self.scopes.push(Default::default());
    }

    /// Pops a scope, returning the symbol table
    /// to what it was before.
    ///
    /// Panics if no scope is on the stack
    pub fn pop_scope(&mut self) {
        let Some(scope) = self.scopes.pop() else {
            panic!("Popped a scope that wasn't pushed");
        };

        for (name, symbol) in scope.old_symbols {
            if let Some(symbol) = symbol {
                self.symbols.insert(name, symbol);
            } else {
                self.symbols.remove(&name);
            }
        }
    }

    /// Inserts a symbol into the current scope,
    /// overwriting any existing symbol with the same name
    pub fn insert(&mut self, name: String, symbol: Id<Symbol>) {
        let old = self.symbols.insert(name.clone(), symbol);

        if let Some(scope) = self.scopes.last_mut() {
            scope.old_symbols.insert(name, old);
        }
    }

    /// Retrieves a symbol from the symbol table
    pub fn get(&self, name: &str) -> Option<Id<Symbol>> {
        self.symbols.get(name).cloned()
    }
}

component!(symbol_tables: SymbolTable);

impl ComputedComponent for SymbolTable {
    fn compute(entity: Id<crate::Entity>, context: &mut HirContext) -> Option<Self> {
        let mut symbol_table = SymbolTable::default();

        // Traverse the namespace hierarchy
        // and add the symbols to the symbol table
        let mut entities_to_traverse = VecDeque::new();
        entities_to_traverse.push_back(entity);

        while let Some(some_namespace_id) = entities_to_traverse.pop_front() {
            let namespace = context.try_get_computed::<Namespace>(some_namespace_id)?;
            let symbols = namespace.symbols.clone();

            let ancestors = Self::get_ancestors(some_namespace_id.as_base(), context);


            // Add the symbols if they don't already exist
            // We support shadowing, so we don't need to check for duplicates
            for symbol_id in symbols.into_iter() {
                // Where is the symbol visible from?
                let Some(VisibleWithin(scope)) = context.try_get_computed::<VisibleWithin>(symbol_id) else {
                    panic!("internal compiler error: couldn't calculate visibility");
                };

                // If we aren't in a scope where the symbol is visible,
                // don't add it
                if !ancestors.contains(&scope) {
                    continue;
                }

                let symbol = context.get(symbol_id);
                let name = symbol.name.name.clone();

                symbol_table.insert(name, symbol_id);
            }

            // Go through imports and add them to the symbol table
            let imports = context.children(some_namespace_id)
                .iter()
                .cloned()
                .filter_map(|id| context.cast_id::<Import>(id))
                .collect_vec();

            for import in imports {
                Self::add_symbols_from_import(import, &mut symbol_table, context);
            }

            if let Some(parent_id) = context.get(some_namespace_id.as_base()).parent {
                entities_to_traverse.push_back(parent_id);
            }
        }

        return Some(symbol_table);
    }
}

impl SymbolTable {
    fn add_symbols_from_import(import_id: Id<Import>, symbol_table: &mut SymbolTable, context: &mut HirContext) {
        let import = context.get(import_id);
        let namespace_id = import.namespace;

        let namespace = context.try_get_computed::<Namespace>(namespace_id)
            .expect("internal compiler error: can only import namespaces");
        let symbols = namespace.symbols.clone();

        // We're looking at 4-6 ancestors on average, so its faster to use
        // a Vec than a HashSet
        let ancestors = Self::get_ancestors(import_id.as_base(), context);

        // Add the symbols if they don't already exist
        // We support shadowing, so we don't need to check for duplicates
        for symbol_id in symbols.into_iter() {
            // Where is the symbol visible from?
            let Some(VisibleWithin(scope)) = context.try_get_computed::<VisibleWithin>(symbol_id) else {
                panic!("internal compiler error: couldn't calculate visibility");
            };

            // If we aren't in a scope where the symbol is visible,
            // don't add it
            if !ancestors.contains(&scope) {
                continue;
            }

            let symbol = context.get(symbol_id);
            let name = symbol.name.name.clone();

            symbol_table.insert(name, symbol_id);
        }
    }

    /// Return a list of the ancestors of an entity
    fn get_ancestors(entity: Id<Entity>, context: &HirContext) -> Vec<Id<Entity>> {
        let mut ancestors = vec![entity];

        let mut current = entity;
        while let Some(parent) = context.parent(current) {
            ancestors.push(parent);
            current = parent;
        }


        return ancestors;
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            old_symbols: HashMap::new(),
        }
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self {
            symbols: HashMap::new(),
            scopes: Vec::new(),
        }
    }
}
