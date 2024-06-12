use std::sync::atomic::AtomicU64;

/// An entity is the base type for anything in the HIR.
/// 
/// An entity can be either cached, or uncached.
pub trait Entity {
    /// Get the unique identifier for this entity
    fn id(&self) -> Id<Self>;
}

/// A unique identifier for an entity
/// 
/// This identifier is unique across the entire compilation,
/// and contains type information to prevent mixing up identifiers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Id<T: Entity>(u64);

impl<T: Entity> Id<T> {
    /// Create a new unique identifier
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        Id(NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    }
}