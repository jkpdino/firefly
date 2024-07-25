use std::collections::VecDeque;

use crate::{ComputedComponent, Id};

use super::{Passthrough, Symbol};

// todo!: add imports

#[derive(Clone, Debug)]
pub struct Namespace {
    pub symbols: Vec<Id<Symbol>>,
}

component!(namespaces: Namespace);

impl ComputedComponent for Namespace {
    fn compute(entity: Id<crate::Entity>, context: &mut crate::HirContext) -> Option<Self> {
        let mut symbols = vec![];

        let mut namespaces = VecDeque::new();
        namespaces.push_back(entity);

        while let Some(entity) = namespaces.pop_front() {
            let children = context.children(entity);

            for child in children.iter().cloned() {
                if let Some(sym) = context.cast_id::<Symbol>(child) {
                    symbols.push(sym);
                }

                if let Some(passthrough) = context.cast_id::<Passthrough>(child) {
                    namespaces.push_back(passthrough.as_base());
                }
            }
        }

        return Some(Namespace { symbols });
    }
}
