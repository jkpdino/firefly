use crate::entity::{Entity, Id};


/// Represents a polymorphic object
/// 
/// Anything that is polymorphic must implement this trait.
/// 
/// Polymorphic objects can be monomorphized, and track
/// their monomorphizations.
pub trait Polymorphic: Entity {

}

pub struct Monomorph<T> where T: Polymorphic
{
    id: Id<T>,
}