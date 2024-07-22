use crate::Id;

use super::Symbol;

// todo!: add imports

pub struct Namespace {
    pub symbols: Vec<Id<Symbol>>,
}

component!(namespaces: Namespace);
