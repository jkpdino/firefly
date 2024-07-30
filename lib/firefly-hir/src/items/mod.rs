use crate::{ty::Ty, value::Value, EntityKind, Id};

#[derive(Clone, Debug)]
pub struct StructDef {
    pub id: Id<StructDef>,
}

component!(base(EntityKind::StructDef) structs: StructDef);

#[derive(Clone, Debug)]
pub struct Field {
    pub id: Id<Field>,
}

component!(base(EntityKind::Field) fields: Field);

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

#[derive(Clone, Debug)]
pub struct Global {
    pub id: Id<Global>,
    pub ty: Ty,
    pub default_value: Value
}

component!(base(EntityKind::Global) globals: Global);