use crate::{Name, Visibility};

/// Having a symbol makes an item referencable
#[derive(Clone, Debug)]
pub struct Symbol {
    pub name: Name,
    pub visibility: Visibility,
    pub is_static: bool,
}

component!(symbols: Symbol);

/// An entity with passthrough will give all its symbols to a namespace
#[derive(Clone, Debug)]
pub struct Passthrough;

component!(passthroughs: Passthrough);