use crate::{EntityKind, Id};

pub struct StructDef {
    id: Id<StructDef>,
}

component!(base(EntityKind::StructDef) structs: StructDef);
