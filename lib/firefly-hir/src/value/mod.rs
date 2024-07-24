use firefly_span::Span;

use crate::{
    entity::{Entity, Id},
    ty::Ty,
};

pub enum LiteralValue {
    Integer(String),
}

pub enum ValueKind {
    Unit,
    Tuple(Vec<Value>),
    Literal(LiteralValue),
    Invoke(Box<Value>, ()),
}

pub struct Value {
    //id: Id<Value>,
    kind: ValueKind,
    ty: Ty,
    span: Span,
}

/*impl Entity for Value {
    fn id(&self) -> Id<Value> {
        self.id
    }
}*/