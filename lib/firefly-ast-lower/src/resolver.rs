// Create a quick-and-dirty scheme to create a namespace tree from the AST
// Eventually, this will be replaced by a more sophisticated scheme that
// generalizes to all AST nodes.

use firefly_ast::item::Item;
use firefly_hir::{
    resolve::{Namespace, Symbol},
    Entity, Id, Name,
};
use firefly_span::Spanned;
use itertools::Itertools;

use crate::AstLowerer;

impl AstLowerer {
    pub fn resolve_pass(&mut self, ast: &[Item]) {
        let root = self.context.root();

        self.resolve_items(ast, root.as_base());
    }

    fn resolve_items(&mut self, items: &[Item], parent: Id<Entity>) {
        // Create a symbol for each item
        let mut symbols = Vec::new();

        for item in items {
            let (symbol, id) = match item {
                Item::Func(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    (Symbol { name, visibility }, item.id.as_base())
                }

                Item::StructDef(Spanned { item, .. }) => {
                    let name = self.lower_name(&item.name);
                    let visibility = self.lower_visibility(&item.visibility);

                    self.resolve_items(&item.items, item.id.as_base());

                    (Symbol { name, visibility }, item.id.as_base())
                }

                _ => continue,
            };

            let symbol_id = self.context.add_component(id, symbol);
            symbols.push(symbol_id);

            // Link it to the parent
            self.context.link(parent, id);
        }

        // Now create a namespace
        let namespace = Namespace { symbols };
        self.context.add_component(parent, namespace);
    }
}
