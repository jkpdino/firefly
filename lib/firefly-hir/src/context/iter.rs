use std::collections::VecDeque;

use crate::{Entity, Id};

use super::HirContext;

/// An iterator over all entities in a `HirContext`
/// 
/// Iterates over all entities in a breadth-first order
pub struct HirContextEntityIter<'a> {
    context: &'a HirContext,
    queue: VecDeque<Id<Entity>>,
}

impl<'a> HirContextEntityIter<'a> {
    pub(super) fn new(context: &'a HirContext) -> Self {
        let mut queue = VecDeque::new();
        queue.push_back(context.root().as_base());
        Self { context, queue }
    }
}

impl<'a> Iterator for HirContextEntityIter<'a> {
    type Item = Id<Entity>;

    fn next(&mut self) -> Option<Self::Item> {
        let id = self.queue.pop_front()?;

        for child in self.context.children(id) {
            self.queue.push_back(*child);
        }

        Some(id)
    }
}