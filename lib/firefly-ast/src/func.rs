use firefly_hir::Id;
use firefly_span::{Span, Spanned};

use crate::{stmt::CodeBlock, ty::Ty, Name, Visibility};

#[derive(Debug)]
pub struct FuncParam {
    pub label: Option<Name>,
    pub name: Name,
    pub ty: Spanned<Ty>,
}

#[derive(Debug)]
pub struct FuncSignature {
    pub params: Vec<Spanned<FuncParam>>,
    pub return_ty: Option<Spanned<Ty>>,
}

#[derive(Debug)]
pub struct Func {
    pub visibility: Option<Spanned<Visibility>>,
    pub static_kw: Option<Span>,
    pub name: Name,
    pub signature: FuncSignature,
    pub body: CodeBlock,
    pub id: Id<firefly_hir::func::Func>,
}

impl Func {
    pub fn new(
        visibility: Option<Spanned<Visibility>>,
        static_kw: Option<Span>,
        name: Name,
        params: Vec<Spanned<FuncParam>>,
        return_ty: Option<Spanned<Ty>>,
        body: CodeBlock,
    ) -> Self {
        Self {
            visibility,
            static_kw,
            name,
            signature: FuncSignature { params, return_ty },
            body,
            id: Id::default(),
        }
    }
}

impl FuncParam {
    pub fn new(label: Option<Name>, name: Name, ty: Spanned<Ty>) -> Self {
        Self { label, name, ty }
    }
}
