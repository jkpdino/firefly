use super::Ty;

#[derive(Debug, Clone)]
pub struct HasType {
    // todo: make this a polymorphic
    pub ty: Ty,
}

component!(has_types: HasType);
