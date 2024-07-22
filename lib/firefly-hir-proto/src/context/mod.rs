use crate::{
    property::{HasProperty, Property},
    Entity, Id,
};

pub struct HirContext {}

impl HirContext {
    /// Creates a new instance of this entity
    pub fn create<T: Entity>(&self, entity: T) {
        todo!()
    }

    /// Retrieves an entity from the Context
    pub fn get<T: Entity>(&self, id: Id<T>) -> &T {
        T::get(id, self)
    }

    /// Gets a property for an entity
    pub fn get_property<E, P>(&mut self, id: Id<E>) -> Option<&P>
    where
        P: Property,
        E: HasProperty<P> + Entity,
    {
        todo!()
    }

    /// Sets a property for an entity
    pub fn set_property<E, P>(&mut self, id: Id<E>, prop: P)
    where
        P: Property,
        E: HasProperty<P> + Entity,
    {
        todo!()
    }
}
