use firefly_span::Spanned;

use crate::{func::Func, import::Import, module::Module, struct_def::{Field, StructDef}};

#[derive(Debug)]
pub enum Item {
    Func(Spanned<Func>),
    Field(Spanned<Field>),
    StructDef(Spanned<StructDef>),
    Module(Spanned<Module>),
    Import(Spanned<Import>)
}