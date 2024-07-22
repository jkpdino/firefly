use std::collections::HashMap;

use firefly_hir::{Name, Visibility};
use namespace::Namespace;
use symbol_table::SymbolTable;

mod import;
mod namespace;
mod symbol;
mod symbol_table;

pub use namespace::NamespaceId;
pub use symbol::{Symbol, SymbolDef};

/*
Two ways to do this:

Have a Resolver struct holding each namespace

or

Extend the entity system to include namespaces

I think I'll go with the first option for now
and then retrofit the second in
*/

pub struct Resolver {
    namespaces: HashMap<NamespaceId, Namespace>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            namespaces: HashMap::new(),
        }
    }

    /// Create a new namespace with the specified parent
    pub fn create_namespace(&mut self, id: NamespaceId, parent: Option<NamespaceId>) {
        assert!(
            !self.namespaces.contains_key(&id),
            "namespace already exists"
        );

        let namespace = Namespace {
            id,
            parent,
            imports: Vec::new(),
            symbols: Vec::new(),
        };

        self.namespaces.insert(id, namespace);
    }

    /// Create a symbol in the specified namespace
    pub fn create_symbol(
        &mut self,
        namespace_id: NamespaceId,
        visibility: Visibility,
        name: Name,
        symbol_def: SymbolDef,
    ) {
        let namespace = self
            .namespaces
            .get_mut(&namespace_id)
            .expect("internal compiler error: namespace not found");

        let symbol = Symbol {
            visibility,
            name,
            symbol_def,
        };

        namespace.symbols.push(symbol);
    }
}
