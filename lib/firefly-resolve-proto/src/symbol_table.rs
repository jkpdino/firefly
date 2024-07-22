//! todo: optimize this code

use std::{collections::HashMap, fmt::Display};

use crate::symbol::{Symbol, SymbolDef};

pub struct Scope {
    symbols: HashMap<String, SymbolDef>,
}

/// Represents a symbol table used for name resolution.
pub struct SymbolTable {
    scopes: Vec<Scope>,
}

impl SymbolTable {
    /// Creates a new symbol table with an initial scope.
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![Scope::new()],
        }
    }

    /// Pushes a new scope onto the symbol table.
    pub fn push_scope(&mut self) {
        self.scopes.push(Scope::new());
    }

    /// Pops the topmost scope from the symbol table.
    /// If there is only one scope remaining, it will not be popped.
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Inserts a symbol into the current scope, overwriting any existing symbol with the same name.
    pub fn insert(&mut self, name: String, symbol: SymbolDef) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, symbol);
        }
    }

    /// Inserts a symbol into the current scope, but does not overwrite any existing symbol with the same name.
    pub fn insert_no_overwrite(&mut self, name: String, symbol: SymbolDef) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.get(&name).is_none() {
                scope.insert(name, symbol);
            }
        }
    }

    /// Retrieves a symbol from the symbol table by name.
    /// Searches the scopes in reverse order (from top to bottom).
    pub fn get(&self, name: &str) -> Option<&SymbolDef> {
        for scope in self.scopes.iter().rev() {
            if let Some(symbol) = scope.get(name) {
                return Some(symbol);
            }
        }
        None
    }

    pub fn display_types(&self) -> DisplayTypes {
        DisplayTypes { table: self }
    }
}

impl Scope {
    /// Creates a new scope with no symbols.
    pub fn new() -> Self {
        Scope {
            symbols: HashMap::new(),
        }
    }

    /// Inserts a symbol into the scope, overwriting any existing symbol with the same name.
    pub fn insert(&mut self, name: String, symbol: SymbolDef) {
        self.symbols.insert(name, symbol);
    }

    /// Retrieves a symbol from the scope by name.
    pub fn get(&self, name: &str) -> Option<&SymbolDef> {
        self.symbols.get(name)
    }
}

pub struct DisplayTypes<'a> {
    table: &'a SymbolTable,
}

impl Display for DisplayTypes<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for scope in self.table.scopes.iter().rev() {
            for (name, symbol) in &scope.symbols {
                match symbol {
                    SymbolDef::Func(_) => {
                        writeln!(f, "{}: Function", name)?;
                    }
                    SymbolDef::StructDef(_) => {
                        writeln!(f, "{}: Struct", name)?;
                    }
                }
            }
        }
        Ok(())
    }
}
