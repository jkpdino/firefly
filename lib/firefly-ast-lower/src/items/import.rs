use firefly_ast::{import::Import as AstImport, Path};
use firefly_hir::{items::Module, resolve::{Import as HirImport, ImportRequest, Symbol}, Id};
use itertools::Itertools;

use crate::{errors::{ModuleError, SymbolError}, AstLowerer};

impl AstLowerer {
    pub fn lower_import(&mut self, import: &AstImport) {
        let Some(module) = self.resolve_module(&import.module) else {
            return;
        };

        let alias = import.alias.as_ref().map(|alias| self.lower_name(alias));

        let symbols = import.symbol_list
            .as_ref()
            .map(|symbol_list| symbol_list.symbols.iter()
            .map(|sym| ImportRequest {
                name: self.lower_name(&sym.name),
                alias: sym.alias.as_ref().map(|alias| self.lower_name(&alias)),
            }).collect_vec());

        self.context.create(HirImport {
            id: import.id,
            namespace: module.as_base(),
            alias,
            symbols
        });
    }

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