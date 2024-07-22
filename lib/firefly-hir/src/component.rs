use std::collections::HashMap;

use crate::entity::{Entity, EntityKind, Id};

pub trait Component {}

pub trait BaseComponent: Component + Sized {
    const ENTITY_KIND: EntityKind;

    fn id(&self) -> Id<Self>;
}

pub trait AccessComponent<C: Component> {
    fn get_components(&self) -> &HashMap<Id<Entity>, C>;
    fn get_components_mut(&mut self) -> &mut HashMap<Id<Entity>, C>;
}
