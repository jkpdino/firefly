use firefly_hir::ty::{Ty as HirTy, TyKind as HirTyKind};
use firefly_interpret::ir::ty::{Ty as VirTy, TyKind as VirTyKind};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_ty(&self, hir_ty: &HirTy) -> VirTy {
        let kind =
        match &hir_ty.kind {
            HirTyKind::Unit => VirTyKind::Void,

            HirTyKind::Integer => VirTyKind::Integer,
            HirTyKind::String => VirTyKind::String,
            HirTyKind::Bool => VirTyKind::Bool,
            HirTyKind::Float => VirTyKind::Float,

            HirTyKind::Tuple(items) => {
                let items = items.iter().map(|item| self.lower_ty(item)).collect();

                VirTyKind::Tuple(items)
            }
            HirTyKind::StructDef(id) => {
                let vir_id = self.struct_map.get(id).unwrap();

                VirTyKind::Struct(*vir_id)
            }
        };

        VirTy::new(kind)
    }
}