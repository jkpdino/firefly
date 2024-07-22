use std::collections::HashMap;

use crate::{HirContext, Id};

use super::{Namespace, Symbol};

/// Stores a delta so that scopes can quickly be restored
struct Scope {
    old_symbols: HashMap<String, Option<Id<Symbol>>>,
}

/// Associates symbols with names, provides for
/// quick lookup
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
    pub fn get(&self, name: &str) -> Option<&Id<Symbol>> {
        self.symbols.get(name)
    }

    pub fn get_for_namespace(id: Id<Namespace>, context: &HirContext) -> SymbolTable {
        let mut symbol_table = SymbolTable::default();

        // Traverse the namespace hierarchy
        // and add the symbols to the symbol table
        let mut namespace_id = Some(id);
        while let Some(some_namespace_id) = namespace_id {
            let namespace = context.get(some_namespace_id);

            // Add the symbols if they don't already exist
            // We support shadowing, so we don't need to check for duplicates
            for symbol_id in namespace.symbols.iter().cloned() {
                let symbol = context.get(symbol_id);
                let name = symbol.name.name.clone();

                symbol_table.insert(name, symbol_id);
            }

            // todo: add imports
            let parent_id = context.get(some_namespace_id.as_base()).parent;
            namespace_id = parent_id.and_then(|parent| context.cast_id(parent));
        }

        return symbol_table;
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
