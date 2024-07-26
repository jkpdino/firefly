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

/// A ComponentConstructor allows for creating components
/// with attached components
pub trait ComponentConstructor: Sized {
    type Base: BaseComponent;

    fn base_id(&self) -> Id<Self::Base>;
    fn create(self, context: &mut HirContext) -> Self::Base;
}

impl<T: BaseComponent> ComponentConstructor for T {
    type Base = T;

    fn base_id(&self) -> Id<Self::Base> {
        self.id()
    }

    fn create(self, _context: &mut HirContext) -> Self::Base {
        self
    }
}

impl<B: BaseComponent, C1: Component> ComponentConstructor for (B, C1)
    where HirContext: AccessComponent<C1>
{
    type Base = B;

    fn base_id(&self) -> Id<Self::Base> {
        self.0.id()
    }

    fn create(self, context: &mut HirContext) -> Self::Base {
        let id = self.0.id();

        context.add_component(id, self.1);

        return self.0
    }
}

impl<B: BaseComponent, C1: Component, C2: Component> ComponentConstructor for (B, C1, C2)
    where HirContext: AccessComponent<C1>,
          HirContext: AccessComponent<C2>
{
    type Base = B;

    fn base_id(&self) -> Id<Self::Base> {
        self.0.id()
    }

    fn create(self, context: &mut HirContext) -> Self::Base {
        let id = self.0.id();

        context.add_component(id, self.1);
        context.add_component(id, self.2);

        return self.0
    }
}