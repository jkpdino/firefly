use crate::{ty::Ty, EntityKind, Id};

pub struct StructDef {
    pub id: Id<StructDef>,
}

component!(base(EntityKind::StructDef) structs: StructDef);

#[derive(Clone)]
pub struct TypeAlias {
    pub id: Id<TypeAlias>,

    pub ty: Ty,
}

component!(base(EntityKind::TypeAlias) typealiases: TypeAlias);

pub struct Module {
    pub id: Id<Module>,
}

component!(base(EntityKind::Module) modules: Module);