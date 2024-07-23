use std::collections::HashMap;

use crate::{
    entity::{Entity, EntityKind, Id},
    HirContext,
};

pub trait Component {}

/// A BaseComponent is the essential component for an entity
pub trait BaseComponent: Component + Sized {
    const ENTITY_KIND: EntityKind;

    fn id(&self) -> Id<Self>;
}

/// A ComputedComponent computes its value when accessed.
/// It can depend on other components
pub trait ComputedComponent: Component + Sized {
    fn compute(entity: Id<Entity>, context: &mut HirContext) -> Option<Self>;
}

pub trait AccessComponent<C: Component> {
    fn get_components(&self) -> &HashMap<Id<Entity>, C>;
    fn get_components_mut(&mut self) -> &mut HashMap<Id<Entity>, C>;
}
