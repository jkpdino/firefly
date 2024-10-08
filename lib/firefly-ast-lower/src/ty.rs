use firefly_ast::ty::Ty as AstTy;
use firefly_hir::{
    items::TypeAlias, resolve::SymbolTable, ty::{HasType, Ty as HirTy, TyKind as HirTyKind}, Entity, Id
};
use firefly_span::Spanned;
use itertools::Itertools;

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_ty(&mut self, ty: &Spanned<AstTy>, parent: Id<Entity>, symbol_table: &SymbolTable) -> HirTy {
        let kind = match &ty.item {
            AstTy::Tuple(items) if items.is_empty() => HirTyKind::Unit,

            AstTy::Tuple(items) if items.len() == 1 => {
                self.lower_ty(&items[0], parent, symbol_table).kind
            }

            AstTy::Tuple(items) => {
                let items = items
                    .iter()
                    .map(|item| self.lower_ty(item, parent, symbol_table))
                    .collect_vec();

                HirTyKind::Tuple(items)
            }

            AstTy::Path(path) => match self.resolve_type(path, parent, symbol_table) {
                Some(ty) => ty.kind,
                None => HirTyKind::Unit,
            },

            AstTy::Error => unreachable!()
        };

        HirTy::new(kind, ty.span)
    }

    pub fn resolve_type_aliases(&mut self) {
        // todo: Can't handle recursive type aliases yet
        self.context.search_for::<TypeAlias>(|id, context| {
            let TypeAlias { id, ty } = context.get(id).clone();

            context.add_component(id, HasType {
                ty,
            });
        });
    }
}
