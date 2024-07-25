use std::fmt::Display;

use crate::{func::{Callable, Func}, items::{Module, StructDef, TypeAlias}, resolve::{Import, Namespace, StaticMemberTable, Symbol, SymbolTable}, stmt::CodeBlock, ty::{HasType, Ty}, value::HasValue, Entity, Id, Root};

use super::HirContext;

macro_rules! for_each_component {
    ($name:ident in $entity:expr, $ctx:expr, ($($t:ty),*), $code:block) => {
        $(
            if let Some($name) = $ctx.try_get::<$t>($entity) {
                $code
            }
        )*
    };
}

pub struct DisplayContext<'a> {
    pub(super) context: &'a HirContext,
    pub(super) node: Id<Entity>,
    pub(super) level: usize,
}

impl Display for DisplayContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = "  ".repeat(self.level);
        let newline_prefix = format!("\n  {prefix}");
        let entity = self.context.get(self.node);

        writeln!(f, "{prefix}{:?}:", entity.kind)?;

        for_each_component!(
            com in self.node,
            self.context,
            (Root, Func, Module, StructDef, TypeAlias, Ty, CodeBlock, HasType, HasValue, Callable, Symbol, Import, Namespace, SymbolTable, StaticMemberTable),
            {
                let com = format!("{com:?}").replace("\n", &newline_prefix);
                println!("  {prefix}{com}");
            }
        );

        let children = self.context.children(self.node);
        let display_children = children.iter()
            .map(|child| {
                DisplayContext {
                    context: self.context,
                    node: *child,
                    level: self.level + 1
                }
            });

        for child in display_children {
            write!(f, "{child}")?;
        }

        Ok(())
    }
}