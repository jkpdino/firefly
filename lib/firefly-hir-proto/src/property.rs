use crate::{entity::AnyEntity, Entity, HirContext, Id};

pub trait Property {
    fn get(id: Id<AnyEntity>, context: &HirContext) -> Option<&Self>;
}

pub trait HasProperty<T: Property>: Entity {}
