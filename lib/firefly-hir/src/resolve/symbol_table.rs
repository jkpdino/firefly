use std::collections::{HashMap, VecDeque};

use crate::{ComputedComponent, HirContext, Id};

use super::{Import, Namespace, Symbol};

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
        entities_to_traverse.push_back((entity, true));

        while let Some((some_namespace_id, follow_imports)) = entities_to_traverse.pop_front() {
            let namespace = context.try_get_computed::<Namespace>(some_namespace_id)?;
            let symbols = namespace.symbols.clone();

            // Add the symbols if they don't already exist
            // We support shadowing, so we don't need to check for duplicates
            for symbol_id in symbols.into_iter() {
                let symbol = context.get(symbol_id);
                let name = symbol.name.name.clone();

                symbol_table.insert(name, symbol_id);
            }

            if follow_imports {
                // Go through imports and add them to the symbol table
                let imports = context.children(some_namespace_id)
                    .iter()
                    .cloned()
                    .filter_map(|id| context.cast_id::<Import>(id))
                    .map(|id| context.get(id))
                    .collect::<Vec<_>>();

                for import in imports {
                    entities_to_traverse.push_back((import.namespace, false))
                }

                if let Some(parent_id) = context.get(some_namespace_id.as_base()).parent {
                    entities_to_traverse.push_back((parent_id, true));
                }
            }
        }

        return Some(symbol_table);
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
