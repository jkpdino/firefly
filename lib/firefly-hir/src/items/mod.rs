use crate::{ty::Ty, EntityKind, Id};

#[derive(Clone, Debug)]
pub struct StructDef {
    pub id: Id<StructDef>,
}

component!(base(EntityKind::StructDef) structs: StructDef);

#[derive(Clone, Debug)]
pub struct TypeAlias {
    pub id: Id<TypeAlias>,

    pub ty: Ty,
}

component!(base(EntityKind::TypeAlias) typealiases: TypeAlias);

#[derive(Clone, Debug, Default)]
pub struct Module {
    pub id: Id<Module>,
}

component!(base(EntityKind::Module) modules: Module);

#[derive(Clone, Debug, Default)]
pub struct SourceFile {
    pub id: Id<SourceFile>
}

component!(base(EntityKind::SourceFile) source_files: SourceFile);