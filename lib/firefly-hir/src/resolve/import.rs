use crate::{Entity, EntityKind, Id};

#[derive(Debug)]
pub struct Import {
    pub id: Id<Import>,
    pub namespace: Id<Entity>,
}

component!(base(EntityKind::Import) imports: Import);