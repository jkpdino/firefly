use firefly_span::Spanned;

use crate::{item::Item, Name, Visibility};

#[derive(Debug)]
pub struct StructDef {
    pub visibility: Option<Spanned<Visibility>>,
    pub name:       Name,
    
    pub items:      Vec<Item>
}

#[derive(Debug)]
pub struct Field {
    pub visibility: Option<Spanned<Visibility>>,
    pub name:       Name,
    pub ty:         Spanned<crate::ty::Ty>,
}