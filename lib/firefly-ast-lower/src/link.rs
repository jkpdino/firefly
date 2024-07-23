// Create a quick-and-dirty scheme to create a namespace tree from the AST
// Eventually, this will be replaced by a more sophisticated scheme that
// generalizes to all AST nodes.

use firefly_ast::item::Item;
use firefly_hir::{
    resolve::Symbol,
    Entity, Id,
};
use firefly_span::Spanned;

use crate::AstLowerer;

impl AstLowerer {
    pub fn link_pass(&mut self, ast: &[Item]) {
        let root = self.context.root();

        self.link_items(ast, root.as_base());
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
}
