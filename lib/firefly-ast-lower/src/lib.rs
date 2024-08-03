use std::sync::Arc;

use firefly_ast::{item::Item, Visibility};
use firefly_errors::emitter::Emitter;
use firefly_hir::{ty::Ty, value::Value, Entity, HirContext, Id, IntoDiagnostic};
use firefly_span::{Span, Spanned};

mod items;
mod link;
mod resolve;
mod ty;
mod util;
mod stmt;
mod value;
pub mod errors;

pub struct AstLowerer {
    context: HirContext,
    pub(crate) self_value: Option<Value>,
}

impl AstLowerer {
    pub fn new(emitter: Arc<Emitter>) -> AstLowerer {
        let mut context = HirContext::new(&emitter);
        firefly_lang::create_lang_module(&mut context);

        let mut lowerer = Self { context, self_value: None };

        lowerer.resolve_type_aliases();

        return lowerer;
    }

    pub fn lower_item_defs(&mut self, items: &[Item]) {
        for item in items {
            self.lower_item_def(item);
        }
    }

    pub fn lower_item_codes(&mut self, items: &[Item]) {
        for item in items {
            self.lower_item_code(item);
        }
    }

    fn lower_item_def(&mut self, item: &Item) {
        let item: &dyn Lower = match item {
            Item::Func(Spanned { item, .. }) => item,
            Item::Field(Spanned { item, .. }) => item,
            Item::Import(Spanned { item, .. }) => item,
            Item::StructDef(Spanned { item, .. }) => {
                self.lower_item_defs(&item.items);
                item
            },
            Item::Module(_) => {
                return
            }
            Item::Error => return,
        };

        let id = item.id();
        let parent = self.context.parent(id).unwrap();

        item.lower_def(parent, self);
    }

    fn lower_item_code(&mut self, item: &Item) {
        let item: &dyn Lower = match item {
            Item::Func(Spanned { item, .. }) => item,
            Item::Field(Spanned { item, .. }) => item,
            Item::Import(Spanned { item, .. }) => item,
            Item::StructDef(Spanned { item, .. }) => {
                self.lower_item_codes(&item.items);
                item
            },
            Item::Module(_) => {
                return
            }
            Item::Error => return,
        };

        let id = item.id();
        let parent = self.context.parent(id).unwrap();

        item.lower_code(parent, self);
    }

    pub fn context(&self) -> &HirContext {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut HirContext {
        &mut self.context
    }

    pub fn emit(&self, diagnostic: impl IntoDiagnostic) {
        self.context.emit(diagnostic);
    }
}

pub trait Lower {
    /// Returns the id of the AST node
    fn id(&self) -> Id<Entity>;

    /// Gets a symbol referring to the AST node
    fn get_symbol(&self) -> Option<SymbolDesc>;

    /// Add HasType or HasValue to a node
    fn get_type(&self) -> Option<Ty> { None }

    /// Lowers definitions
    fn lower_def(&self, parent: Id<Entity>, lowerer: &mut AstLowerer);

    /// Lowers code
    fn lower_code(&self, parent: Id<Entity>, lowerer: &mut AstLowerer);
}

pub struct SymbolDesc {
    pub name: Spanned<String>,
    pub visibility: Option<Spanned<Visibility>>,
    pub static_kw: Option<Span>,
}