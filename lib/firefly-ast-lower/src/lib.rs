use firefly_ast::item::Item;
use firefly_hir::{Entity, HirContext, Id};
use firefly_span::Spanned;

mod items;
mod link;
mod resolve;
mod ty;
mod util;
mod stmt;
mod value;

pub struct AstLowerer {
    context: HirContext,
}

impl AstLowerer {
    pub fn new() -> AstLowerer {
        let mut context = HirContext::new();
        firefly_lang::create_lang_module(&mut context);

        Self { context }
    }

    pub fn lower(&mut self, items: &[Item]) {
        let parent = self.context.root().as_base();

        self.resolve_type_aliases();

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

    pub fn context(&self) -> &HirContext {
        &self.context
    }
}
