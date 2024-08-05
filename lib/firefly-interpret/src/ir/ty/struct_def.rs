use crate::{ir::VirContext, util::{DisplayInContext, UniqueId}};

use super::Ty;

pub struct StructDef {
    pub id:     UniqueId<StructDef>,
    pub name:   String,
    pub fields: Vec<Ty>
}

impl DisplayInContext for StructDef {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &VirContext) -> std::fmt::Result {
        writeln!(f, "struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "{},", context.display(field))?;
        }
        write!(f, "}}")
    }
}