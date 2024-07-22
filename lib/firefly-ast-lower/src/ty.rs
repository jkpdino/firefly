use firefly_ast::ty::Ty as AstTy;
use firefly_hir::ty::{Ty as HirTy, TyKind as HirTyKind};
use firefly_span::Spanned;
use itertools::Itertools;

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_ty(&self, ty: &Spanned<AstTy>) -> HirTy {
        let ty_kind = match &ty.item {
            AstTy::Tuple(items) if items.is_empty() => HirTyKind::Unit,

            AstTy::Tuple(items) => {
                let items = items.iter().map(|item| self.lower_ty(item)).collect_vec();

                HirTyKind::Tuple(items)
            }

            AstTy::Path(path) => {
                todo!()
            }
        };

        todo!()
    }
}
