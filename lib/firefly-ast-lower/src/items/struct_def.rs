use crate::{AstLowerer, Lower, SymbolDesc};
use firefly_ast::struct_def::StructDef as AstStruct;
use firefly_hir::{items::{Field, StructDef as HirStructDef}, ty::{Ty, TyKind}, value::{HasValue, Value, ValueKind}, Entity, Id};
use itertools::Itertools;


impl Lower for AstStruct {
    fn id(&self) -> Id<Entity> {
        self.id.as_base()
    }

    fn get_symbol(&self) -> Option<SymbolDesc> {
        let name = self.name.clone();
        let visibility = self.visibility.clone();

        Some(SymbolDesc { name, visibility, static_kw: None })
    }

    fn get_type(&self) -> Option<firefly_hir::ty::Ty> {
        Some(Ty::new_unspanned(TyKind::StructDef(self.id)))
    }

    fn lower_def(&self, _: Id<Entity>, lowerer: &mut AstLowerer) {
        let fields = lowerer.context()
                            .children(self.id())
                            .iter()
                            .filter_map(|child| lowerer.context().try_get::<Field>(*child))
                            .map(|field| field.ty.clone())
                            .collect_vec();
        
        let my_type = Ty::new_unspanned(TyKind::StructDef(self.id));
        let init_type = Ty::new_unspanned(TyKind::Func(fields, Box::new(my_type)));
        let value = Value {
            kind: ValueKind::InitFor(self.id),
            ty: init_type,
            span: Default::default(),
        };

        lowerer.context_mut().create((
            HirStructDef { id: self.id },
            HasValue { value }
        ));
    }

    fn lower_code(&self, _: Id<Entity>, _: &mut AstLowerer) { }
}