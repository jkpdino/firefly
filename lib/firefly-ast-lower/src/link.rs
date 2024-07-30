// Create a quick-and-dirty scheme to create a namespace tree from the AST
// Eventually, this will be replaced by a more sophisticated scheme that
// generalizes to all AST nodes.

use firefly_ast::item::Item;
use firefly_hir::{
    items::{Module, SourceFile}, resolve::{Passthrough, Symbol}, ty::{HasType, Ty, TyKind}, Entity, Id, Name, Visibility
};
use firefly_span::Spanned;
use itertools::Itertools;

use crate::{errors::ModuleError, AstLowerer};

impl AstLowerer {
    pub fn link_pass(&mut self, ast: &[Item]) {
        let Some(module) = self.get_module(ast) else {
            return;
        };

        // Create a file
        let source_file = self.context.create_with_parent(module, (
            SourceFile::default(),
            Passthrough
        ));

        self.link_items(ast, source_file.as_base());
    }

    fn link_items(&mut self, items: &[Item], parent: Id<Entity>) {
        for item in items {
            let id = match item {
                Item::Func(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    let symbol = Symbol { name, visibility, is_static: true };
                    self.context.add_component(item.id, symbol);

                    item.id.as_base()
                }

                Item::Field(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    let symbol = Symbol { name, visibility, is_static: true };
                    self.context.add_component(item.id, symbol);

                    item.id
                }

                Item::StructDef(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    self.link_items(&item.items, item.id.as_base());

                    let symbol = Symbol { name, visibility, is_static: true };
                    self.context.add_component(item.id, symbol);

                    self.context.add_component(item.id, HasType {
                        ty: Ty::new_unspanned(TyKind::StructDef(item.id))
                    });

                    item.id.as_base()
                }

                Item::Import(Spanned { item, .. }) => {
                    item.id.as_base()
                }

                Item::Module(Spanned { span, .. }) => {
                    if self.context.try_get::<SourceFile>(parent).is_none() {
                        self.emit(ModuleError::ModuleDeclarationInside(*span));
                    }

                    continue;
                }

                Item::Error => continue
            };

            // Link it to the parent
            self.context.link(parent, id);
        }
    }

    fn get_module(&mut self, items: &[Item]) -> Option<Id<Module>> {
        let module_defs = items.iter().filter_map(|item| match item {
            Item::Module(module) => Some(module),
            _ => None
        }).collect_vec();

        let module_def = match &module_defs[..] {
            [] => {
                self.emit(ModuleError::NoModuleFound);
                return None
            }
            [module_def] => *module_def,
            modules @ [..] => { 
                let spans = modules.iter().map(|module| module.span).collect_vec();
                self.emit(ModuleError::MultipleModulesFound(spans));
                return None
            }
        };

        let path = &module_def.item.path;

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

            // Create a new submodule if one doesn't already exist
            else {
                let module = self.context.create_with_parent(current, (
                    Module::default(),
                    Symbol {
                        visibility: Visibility::Public,
                        name: Name::internal(&segment.name.item),
                        is_static: true,
                    }
                ));
    
                current = module.as_base();
            }
        }

        // Return the module we get
        let module = self.context.cast_id::<Module>(current)
            .expect("internal compiler error");
        Some(module)
    }
}
