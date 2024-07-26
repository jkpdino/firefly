use crate::{Entity, EntityKind, Id, Name};

#[derive(Clone, Debug)]
pub struct ImportRequest {
    pub name: Name,
    pub alias: Option<Name>
}

#[derive(Clone, Debug)]
pub struct Import {
    pub id: Id<Import>,
    pub namespace: Id<Entity>,
    pub alias: Option<Name>,
    pub symbols: Option<Vec<ImportRequest>>
}

component!(base(EntityKind::Import) imports: Import);

impl Import {
    pub fn import(id: Id<Import>, namespace: Id<Entity>) -> Self {
        Self { id, namespace, alias: None, symbols: None }
    }

    pub fn import_aliased(id: Id<Import>, namespace: Id<Entity>, alias: Name) -> Self {
        Self { id, namespace, alias: Some(alias), symbols: None }
    }
}