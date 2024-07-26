use firefly_ast::item::Item;
use firefly_hir::HirContext;
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

        let mut lowerer = Self { context };

        lowerer.resolve_type_aliases();

        return lowerer;
    }

    pub fn lower_items(&mut self, items: &[Item]) {
        for item in items {
            self.lower_item(item);
        }
    }

    fn lower_item(&mut self, item: &Item) {
        match item {
            Item::Func(Spanned { item, .. }) => {
                let parent = self.context.parent(item.id.as_base()).unwrap();
                self.lower_func(item, parent);
            }

            Item::StructDef(Spanned { .. }) => {

            }

            Item::Import(Spanned { item, .. }) => {
                self.lower_import(item);
            }

            _ => {}
        }
    }

    pub fn context(&self) -> &HirContext {
        &self.context
    }
}
