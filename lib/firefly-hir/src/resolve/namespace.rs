use crate::{ComputedComponent, Id};

use super::Symbol;

// todo!: add imports

pub struct Namespace {
    pub symbols: Vec<Id<Symbol>>,
}

component!(namespaces: Namespace);

impl ComputedComponent for Namespace {
    fn compute(entity: Id<crate::Entity>, context: &mut crate::HirContext) -> Option<Self> {
        // A namespace is composed of all the children of an entity which
        // have a symbol
        let children = context.children(entity);
        let symbols = children
            .iter()
            .cloned()
            .filter_map(|child| context.cast_id::<Symbol>(child))
            .collect();

        return Some(Namespace { symbols });
    }
}
