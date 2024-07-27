use firefly_hir::Id;
use firefly_hir::resolve::Import as HirImport;

use crate::{Name, Path};

#[derive(Debug, Default)]
pub struct Import {
    pub id: Id<HirImport>,
    pub module: Path,
    pub alias: Option<Name>,
    pub symbol_list: Option<ImportSymbolList>,
}

#[derive(Debug)]
pub struct ImportSymbolList {
    pub symbols: Vec<ImportSymbol>
}

#[derive(Debug)]
pub struct ImportSymbol {
    pub name: Name,
    pub alias: Option<Name>
}

impl Import {
    pub fn new(module: Path, alias: Option<Name>, symbol_list: Option<ImportSymbolList>) -> Self {
        Self {
            id: Id::default(),
            module,
            alias,
            symbol_list,
        }
    }
}
