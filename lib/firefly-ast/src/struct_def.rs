use firefly_hir::Id;
use firefly_span::Spanned;

use crate::{item::Item, Name, Visibility};

#[derive(Debug)]
pub struct StructDef {
    pub visibility: Option<Spanned<Visibility>>,
    pub name: Name,
    pub id: Id<firefly_hir::items::StructDef>,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Field {
    pub visibility: Option<Spanned<Visibility>>,
    pub name: Name,
    pub ty: Spanned<crate::ty::Ty>,
}

impl StructDef {
    pub fn new(visibility: Option<Spanned<Visibility>>, name: Name, items: Vec<Item>) -> Self {
        Self {
            visibility,
            name,
            id: Id::default(),
            items,
        }
    }
}
