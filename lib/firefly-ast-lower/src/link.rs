// Create a quick-and-dirty scheme to create a namespace tree from the AST
// Eventually, this will be replaced by a more sophisticated scheme that
// generalizes to all AST nodes.

use firefly_ast::item::Item;
use firefly_hir::{
    items::Module, resolve::Symbol, Entity, Id, Name, Visibility
};
use firefly_span::Spanned;

use crate::AstLowerer;

impl AstLowerer {
    pub fn link_pass(&mut self, ast: &[Item]) {
        let Some(module) = self.get_module(ast) else {
            println!("error: no module definition found");
            return;
        };

        /* todo: ensure ONE module is found */
        /* todo: ensure no modules are found unless we are in the root */
        /* todo: we need a file to go inside a module */

        self.link_items(ast, module.as_base());
    }

    fn link_items(&mut self, items: &[Item], parent: Id<Entity>) {
        for item in items {
            let id = match item {
                Item::Func(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    let symbol = Symbol { name, visibility };
                    self.context.add_component(item.id, symbol);

                    item.id.as_base()
                }

                Item::StructDef(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    self.link_items(&item.items, item.id.as_base());

                    let symbol = Symbol { name, visibility };
                    self.context.add_component(item.id, symbol);

                    item.id.as_base()
                }

                _ => continue,
            };

            // Link it to the parent
            self.context.link(parent, id);
        }
    }

    fn get_module(&mut self, items: &[Item]) -> Option<Id<Module>> {
        let module_def = items.iter().find_map(|item| match item {
            Item::Module(module) => Some(module),
            _ => None
        })?;

        let path = &module_def.item.path;

        let mut current = self.context.root().as_base();

        for segment in &path.segments {
            let next = self.context.children(current)
                .iter()
                .filter_map(|id| self.context().cast_id::<Symbol>(*id))
                .find(|sym| self.context().get(*sym).name.name == segment.name.item);

            if let Some(next_id) = next {
                if self.context.cast_id::<Module>(next_id).is_none() {
                    println!("error: {} is not a module", segment.name.item);
                    return None;
                }

                current = next_id.as_base();
            }
            else {
                let module = self.context.create(
                    Module { id: Default::default() }
                );
                self.context.add_component(module, Symbol {
                    visibility: Visibility::Public,
                    name: Name {
                        name: segment.name.item.clone(),
                        span: Default::default(),
                    }
                });

                self.context.link(current, module);
                current = module.as_base();
            }
        }

        let module = self.context.cast_id::<Module>(current).expect("internal compiler error");

        Some(module)
    }
}
