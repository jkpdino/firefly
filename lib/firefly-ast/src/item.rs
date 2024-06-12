use firefly_span::Spanned;

use crate::{func::Func, struct_def::{Field, StructDef}};

#[derive(Debug)]
pub enum Item {
    Func(Spanned<Func>),
    Field(Spanned<Field>),
    StructDef(Spanned<StructDef>),
}