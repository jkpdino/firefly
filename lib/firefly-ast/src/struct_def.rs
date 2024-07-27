use firefly_hir::{Entity, Id};
use firefly_span::Spanned;

use crate::{item::Item, value::Value, Name, Visibility};

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
    pub default: Option<Spanned<Value>>,
    pub id: Id<Entity>,
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

impl Field {
    pub fn new(visibility: Option<Spanned<Visibility>>, name: Name, ty: Spanned<crate::ty::Ty>, default: Option<Spanned<Value>>) -> Self {
        Self {
            visibility,
            name,
            ty,
            default,
            id: Id::default(),
        }
    }
}