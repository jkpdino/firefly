use firefly_ast::{import::Import as AstImport, Path};
use firefly_hir::{items::Module, resolve::{Import as HirImport, ImportRequest, Symbol}, Id};
use itertools::Itertools;

use crate::{errors::{ModuleError, SymbolError}, AstLowerer, Lower};

impl AstLowerer {
    fn resolve_module(&mut self, path: &Path) -> Option<Id<Module>> {
        let mut current = self.context.root().as_base();

        for segment in &path.segments {
            let next = self.context.children(current)
                .iter()
                .filter_map(|id| self.context().cast_id::<Symbol>(*id))
                .find(|sym| self.context().get(*sym).name.name == segment.name.item);


            // If the item we're accessing isn't a module,
            // throw an error
            if let Some(next_id) = next {
                if !self.context.has::<Module>(next_id) {
                    self.emit(ModuleError::NotAModule(self.lower_name(&segment.name)));
                    return None;
                }

                current = next_id.as_base();
                continue;
            }

            // If the module doesn't exist, throw an error
            else {
                self.emit(SymbolError::NotFound(segment.name.clone()));
                return None;
            }
        }

        // Return the module we get
        let module = self.context.cast_id::<Module>(current)
            .expect("internal compiler error");
        Some(module)
    }
}

impl Lower for AstImport {
    fn id(&self) -> Id<firefly_hir::Entity> {
        self.id.as_base()
    }

    fn get_symbol(&self) -> Option<crate::SymbolDesc> {
        None
    }

    fn lower_def(&self, _: Id<firefly_hir::Entity>, lowerer: &mut AstLowerer) {
        let Some(module) = lowerer.resolve_module(&self.module) else {
            return;
        };

        let alias = self.alias.as_ref().map(|alias| lowerer.lower_name(alias));

        let symbols = self.symbol_list
            .as_ref()
            .map(|symbol_list| symbol_list.symbols.iter()
            .map(|sym| ImportRequest {
                name: lowerer.lower_name(&sym.name),
                alias: sym.alias.as_ref().map(|alias| lowerer.lower_name(&alias)),
            }).collect_vec());

        lowerer.context_mut().create(HirImport {
            id: self.id,
            namespace: module.as_base(),
            alias,
            symbols
        });
    }

    fn lower_code(&self, _: Id<firefly_hir::Entity>, _: &mut AstLowerer) { }
}