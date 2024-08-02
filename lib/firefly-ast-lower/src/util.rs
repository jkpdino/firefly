use firefly_hir::Name;
use firefly_span::Spanned;

use crate::AstLowerer;

impl AstLowerer {
    pub fn lower_name(&self, name: &Spanned<String>) -> Name {
        Name {
            name: name.item.clone(),
            span: name.span,
        }
    }

    pub fn lower_visibility(
        &self,
        visibility: &Option<Spanned<firefly_ast::Visibility>>,
    ) -> firefly_hir::Visibility {
        let Some(visibility) = visibility else {
            return firefly_hir::Visibility::Internal;
        };

        match visibility.item {
            firefly_ast::Visibility::Private => firefly_hir::Visibility::Private,
            firefly_ast::Visibility::FilePrivate => firefly_hir::Visibility::FilePrivate,
            firefly_ast::Visibility::Internal => firefly_hir::Visibility::Internal,
            firefly_ast::Visibility::Public => firefly_hir::Visibility::Public,
        }
    }
}