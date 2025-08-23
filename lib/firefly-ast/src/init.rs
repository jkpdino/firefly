use firefly_hir::Id;
use firefly_span::Spanned;

use crate::{
    func::{FuncParam, FuncSignature},
    stmt::CodeBlock,
    Visibility,
};

#[derive(Debug)]
pub struct Init {
    pub visibility: Option<Spanned<Visibility>>,
    pub signature: FuncSignature,
    pub body: CodeBlock,
    pub id: Id<firefly_hir::func::Init>,
}

impl Init {
    pub fn new(
        visibility: Option<Spanned<Visibility>>,
        params: Vec<Spanned<FuncParam>>,
        body: CodeBlock,
    ) -> Self {
        Self {
            visibility,
            signature: FuncSignature {
                params,
                return_ty: None,
            },
            body,
            id: Id::default(),
        }
    }
}
