use firefly_span::Spanned;

use crate::{stmt::CodeBlock, ty::Ty, Name, Visibility};

#[derive(Debug)]
pub struct FuncParam {
    pub name: Name,
    pub ty:   Spanned<Ty>,
}

#[derive(Debug)]
pub struct Func {
    pub visibility: Option<Spanned<Visibility>>,
    pub name:       Name,
    pub params:     Vec<Spanned<FuncParam>>,
    pub return_ty:  Spanned<Ty>,
    pub body:       CodeBlock,
}

impl FuncParam {
    pub fn new(name: Name, ty: Spanned<Ty>) -> Self {
        Self {
            name,
            ty,
        }
    }
}