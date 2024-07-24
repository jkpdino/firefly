use crate::{Name, Visibility};

/// Having a symbol makes an item referencable
#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: Name,
    pub visibility: Visibility,
}

component!(symbols: Symbol);
