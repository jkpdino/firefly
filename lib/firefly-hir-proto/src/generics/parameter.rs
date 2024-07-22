use crate::entity::{Entity, Id};

///
/// A generic parameter represents a type parameter
/// in a generic type or function.
/// 
/// The generic parameter has a list of constraints
/// that must be satisfied by the type that is passed
/// to the generic.
/// 
/// The generic parameter can be used like a type.
/// 
pub struct GenericParameter {
    id: Id<GenericParameter>,
}

impl Entity for GenericParameter {
    fn id(&self) -> Id<GenericParameter> {
        self.id
    }
}

pub struct GenericParameterList {
    parameters: Vec<Id<GenericParameter>>
}