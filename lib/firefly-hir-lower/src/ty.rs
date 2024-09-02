use firefly_hir::ty::{Ty as HirTy, TyKind as HirTyKind};
use firefly_mir::ty::{Ty as MirTy, TyKind as MirTyKind};

use crate::HirLowerer;

impl HirLowerer<'_> {
    pub fn lower_ty(&self, hir_ty: &HirTy) -> MirTy {
        let kind =
        match &hir_ty.kind {
            HirTyKind::Unit => MirTyKind::Void,
            HirTyKind::Never => MirTyKind::Void,

            HirTyKind::Integer => MirTyKind::Integer,
            HirTyKind::String => MirTyKind::String,
            HirTyKind::Bool => MirTyKind::Bool,
            HirTyKind::Float => MirTyKind::Float,

            HirTyKind::Func(params, return_ty) => {
                let params = params.iter().map(|p| self.lower_ty(p)).collect();
                let return_ty = self.lower_ty(&return_ty);

                MirTyKind::Func(params, return_ty)
            }

            HirTyKind::Tuple(items) => {
                let items = items.iter().map(|item| self.lower_ty(item)).collect();

                MirTyKind::Tuple(items)
            }
            HirTyKind::StructDef(id) => {
                let mir_id = self.struct_map.get(id).unwrap();

                MirTyKind::Struct(*mir_id)
            }
        };

        MirTy::new(kind)
    }
}