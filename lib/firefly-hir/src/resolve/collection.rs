use crate::Id;

use super::Symbol;

#[derive(Debug, Clone)]
pub struct SymbolCollection {
    pub symbols: Vec<Id<Symbol>>,
}

impl Default for SymbolCollection {
    fn default() -> Self {
        Self {
            symbols: Vec::new(),
        }
    }
}

impl SymbolCollection {
    pub fn new(symbols: Vec<Id<Symbol>>) -> Self {
        Self { symbols }
    }

    pub fn new_single(symbol: Id<Symbol>) -> Self {
        Self::new(vec![symbol])
    }

    pub fn add(&mut self, symbol: Id<Symbol>) {
        if self.symbols.contains(&symbol) {
            return;
        }
        self.symbols.push(symbol);
    }

    pub fn single(&self) -> Option<Id<Symbol>> {
        if self.symbols.len() > 1 {
            return None;
        }

        self.symbols.get(0).copied()
    }

    pub fn is_empty(&self) -> bool {
        self.symbols.is_empty()
    }

    pub fn symbols_matching(&self, predicate: impl Fn(Id<Symbol>) -> bool) -> Self {
        Self::new(
            self.symbols
                .iter()
                .filter(|&&id| predicate(id))
                .copied()
                .collect(),
        )
    }
}
