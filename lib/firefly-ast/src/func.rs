use firefly_hir::Id;
use firefly_span::Spanned;

use crate::{stmt::CodeBlock, ty::Ty, Name, Visibility};

#[derive(Debug)]
pub struct FuncParam {
    pub name: Name,
    pub ty: Spanned<Ty>,
}

#[derive(Debug)]
pub struct FuncSignature {
    pub params:     Vec<Spanned<FuncParam>>,
    pub return_ty:  Spanned<Ty>,
}

#[derive(Debug)]
pub struct Func {
    pub visibility: Option<Spanned<Visibility>>,
    pub name: Name,
    pub signature: FuncSignature,
    pub body: CodeBlock,
    pub id: Id<firefly_hir::func::Func>,
}

impl Func {
    pub fn new(
        visibility: Option<Spanned<Visibility>>,
        name: Name,
        params: Vec<Spanned<FuncParam>>,
        return_ty: Spanned<Ty>,
        body: CodeBlock,
    ) -> Self {
        Self {
            visibility,
            name,
            signature: FuncSignature { params, return_ty },
            body,
            id: Id::default(),
        }
    }
}

impl FuncParam {
    pub fn new(name: Name, ty: Spanned<Ty>) -> Self {
        Self { name, ty }
    }
}
