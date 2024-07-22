use firefly_ast::item::Item;
use firefly_hir::{resolve::Namespace, Entity, HirContext, Id};
use firefly_span::Spanned;

mod items;
mod resolver;
mod ty;
mod util;

pub struct AstLowerer {
    context: HirContext,
}

impl AstLowerer {
    pub fn new() -> AstLowerer {
        Self {
            context: HirContext::new(),
        }
    }

    pub fn lower(&mut self, items: &[Item]) {
        let parent = self.context.root().as_base();

        self.lower_items(items, parent);
    }

    fn lower_items(&mut self, items: &[Item], parent: Id<Entity>) {
        for item in items {
            self.lower_item(item, parent);
        }
    }

    fn lower_item(&mut self, item: &Item, parent: Id<Entity>) {
        match item {
            Item::Func(Spanned { item, .. }) => {
                self.lower_func(item, parent);
            }

            _ => {}
        }
    }
}
