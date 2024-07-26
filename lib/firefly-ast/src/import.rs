use crate::{Name, Path};

#[derive(Debug)]
pub struct Import {
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