use std::collections::HashMap;

use itertools::Itertools;

use crate::{ComputedComponent, Entity, HirContext, Id, ImportError};

use super::{Import, ImportRequest, Namespace, Symbol, SymbolCollection, VisibleWithin};

/// Represents a single scope level in the symbol table.
/// Stores the previous state of symbols before they were shadowed,
/// enabling proper restoration when the scope is popped.
#[derive(Clone, Debug)]
struct Scope {
    old_symbols: HashMap<String, Option<SymbolCollection>>,
}

/// A SymbolTable manages name resolution and scoping in the compiler.
/// It maintains maps symbol names to their corresponding IDs.
#[derive(Clone, Debug)]
pub struct SymbolTable {
    symbols: HashMap<String, SymbolCollection>,
    scopes: Vec<Scope>,
}

impl SymbolTable {
    /// Creates a new nested scope level.
    /// This allows for symbol shadowing while preserving outer scope symbols.
    pub fn push_scope(&mut self) {
        self.scopes.push(Default::default());
    }

    /// Restores the symbol table to its state before the current scope was created.
    /// All symbols defined in the current scope are removed, and shadowed symbols
    /// from outer scopes are restored.
    pub fn pop_scope(&mut self) {
        let Some(scope) = self.scopes.pop() else {
            panic!("Popped a scope that wasn't pushed");
        };

        for (name, symbols) in scope.old_symbols {
            match symbols {
                Some(symbols) => {
                    self.symbols.insert(name, symbols);
                }
                None => {
                    self.symbols.remove(&name);
                }
            }
        }
    }

    /// Adds or updates a symbol in the current scope.
    /// If the symbol already exists, it will be shadowed and the old value
    /// will be restored when the current scope is popped.
    pub fn insert(&mut self, name: String, symbol: Id<Symbol>) {
        let symbols = self.symbols.entry(name.clone()).or_default();
        symbols.add(symbol);

        if let Some(scope) = self.scopes.last_mut() {
            // Store the previous state before modification
            if !scope.old_symbols.contains_key(&name) {
                scope
                    .old_symbols
                    .insert(name.clone(), self.symbols.get(&name).cloned());
            }
        }
    }

    /// Looks up a symbol by name in the current scope and all outer scopes.
    /// Returns None if the symbol is not found in any accessible scope.
    pub fn get(&self, name: &str) -> Option<&SymbolCollection> {
        self.symbols.get(name)
    }
}

component!(symbol_tables: SymbolTable);

impl ComputedComponent for SymbolTable {
    fn compute(entity: Id<crate::Entity>, context: &mut HirContext) -> Option<Self> {
        let mut symbol_table = context
            .parent(entity)
            .and_then(|parent| context.try_get_computed::<SymbolTable>(parent))
            .cloned()
            .unwrap_or_default();

        symbol_table.push_scope();

        let namespace = context.try_get_computed::<Namespace>(entity)?;
        let symbols = namespace.symbols.clone();

        // We're looking at 4-6 ancestors on average, so its faster to use
        // a Vec than a HashSet
        let ancestors = Self::get_ancestors(entity.as_base(), context);

        // Add the symbols if they don't already exist
        // We support shadowing, so we don't need to check for duplicates
        for symbol_id in symbols.into_iter() {
            // Where is the symbol visible from?
            let Some(VisibleWithin(scope)) = context.try_get_computed::<VisibleWithin>(symbol_id)
            else {
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
        let imports = context
            .children(entity)
            .iter()
            .cloned()
            .filter_map(|id| context.cast_id::<Import>(id))
            .collect_vec();

        for import in imports {
            Self::import(import, &mut symbol_table, context)
        }

        return Some(symbol_table);
    }
}

impl SymbolTable {
    /// Processes an import declaration and adds the imported symbols
    /// to the symbol table according to the import rules.
    fn import(import: Id<Import>, symbol_table: &mut SymbolTable, context: &mut HirContext) {
        let import = context.get(import);
        let namespace_id = import.namespace;

        let symbols = import.symbols.clone();

        if let Some(alias) = &import.alias {
            let Some(symbol) = context.cast_id::<Symbol>(namespace_id) else {
                panic!("internal compiler error: module has no symbol");
            };
            symbol_table.insert(alias.name.clone(), symbol);
        } else if symbols.is_none() {
            Self::add_all_symbols(namespace_id, symbol_table, context);
        }

        if let Some(symbols) = symbols {
            Self::add_specific_symbols(namespace_id, symbols, symbol_table, context);
        }
    }

    /// Imports all visible symbols from a namespace into the current scope.
    fn add_all_symbols(
        namespace_id: Id<Entity>,
        symbol_table: &mut SymbolTable,
        context: &mut HirContext,
    ) {
        let Some(namespace) = context.try_get_computed::<Namespace>(namespace_id) else {
            panic!("internal compiler error: no namespace for import")
        };
        let symbols = namespace.symbols.clone();

        // We're looking at 4-6 ancestors on average, so its faster to use
        // a Vec than a HashSet
        let ancestors = Self::get_ancestors(namespace_id.as_base(), context);

        // Add the symbols if they don't already exist
        // We support shadowing, so we don't need to check for duplicates
        for symbol_id in symbols.into_iter() {
            // Where is the symbol visible from?
            let Some(VisibleWithin(scope)) = context.try_get_computed::<VisibleWithin>(symbol_id)
            else {
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

    /// Imports specific requested symbols from a namespace, handling aliases
    /// and visibility checks.
    fn add_specific_symbols(
        namespace_id: Id<Entity>,
        symbols: Vec<ImportRequest>,
        symbol_table: &mut SymbolTable,
        context: &mut HirContext,
    ) {
        // We need to match symbols to their import symbols
        // Create a map to do this on O(n) time
        let mut symbol_map = HashMap::<String, ImportRequest>::new();

        for symbol in symbols {
            if let Some(original) = symbol_map.get(&symbol.name.name) {
                context.emit(ImportError::MultipleImports(
                    original.name.clone(),
                    symbol.name.clone(),
                ));
            }

            symbol_map.insert(symbol.name.name.clone(), symbol);
        }

        let Some(namespace) = context.try_get_computed::<Namespace>(namespace_id) else {
            panic!("internal compiler error: no namespace for import")
        };
        let symbols = namespace.symbols.clone();

        // We're looking at 4-6 ancestors on average, so its faster to use
        // a Vec than a HashSet
        let ancestors = Self::get_ancestors(namespace_id.as_base(), context);

        // Add the symbols if they don't already exist
        // We support shadowing, so we don't need to check for duplicates
        for symbol_id in symbols.into_iter() {
            let symbol = context.get(symbol_id);
            let symbol_name = symbol.name.clone();
            let mut name = symbol.name.name.clone();

            // Check if we are looking for that symbol
            let Some(symbol_req) = symbol_map.remove(&name) else {
                continue;
            };

            // If there's an alias, add that symbol
            if let Some(alias) = symbol_req.alias {
                name = alias.name.clone();
            }

            // Where is the symbol visible from?
            let Some(VisibleWithin(scope)) = context.try_get_computed::<VisibleWithin>(symbol_id)
            else {
                panic!("internal compiler error: couldn't calculate visibility");
            };

            // If we aren't in a scope where the symbol is visible,
            // don't add it
            if !ancestors.contains(&scope) {
                context.emit(ImportError::NotVisible(symbol_name));
            }

            symbol_table.insert(name, symbol_id);
        }

        // Check that we imported all requested symbols
        for ImportRequest { name, .. } in symbol_map.values() {
            context.emit(ImportError::NotFound(name.clone()));
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
