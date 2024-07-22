use crate::{Entity, Id, Name, Visibility};

/// Represents a struct in the IR
#[derive(Debug, Clone)]
pub struct StructDef {
    id: Id<StructDef>,

    visibility: Visibility,
    name: Name,
}

impl Entity for StructDef {
    fn id(&self) -> Id<Self> {
        self.id
    }
}
