mod code_block;

use firefly_span::Span;

use crate::{
    entity::{Entity, Id},
    ty::Ty,
    value::Value,
    Name,
};

pub enum StmtKind {
    Value(Value),
    Bind(Name, Ty, Value),
}

pub struct Stmt {
    pub id: Id<Stmt>,
    pub kind: StmtKind,
    pub span: Span,
}

impl Entity for Stmt {
    fn id(&self) -> Id<Self> {
        self.id
    }
}
