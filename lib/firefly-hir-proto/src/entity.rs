use std::{marker::PhantomData, sync::atomic::AtomicU64};

use crate::HirContext;

/// An entity is the base type for anything in the HIR.
///
/// An entity can be either cached, or uncached.
pub trait Entity: Sized {
    /// Get the unique identifier for this entity
    fn id(&self) -> Id<Self>;

    fn get(id: Id<Self>, context: &HirContext) -> &Self;
}

/// A unique identifier for an entity
///
/// This identifier is unique across the entire compilation,
/// and contains type information to prevent mixing up identifiers.
#[derive(Debug, Eq, Hash)]
pub struct Id<T: Entity>(u64, PhantomData<T>);

impl<T: Entity> Id<T> {
    /// Create a new unique identifier
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);

        Self(
            NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            PhantomData,
        )
    }
}

impl<T: Entity> Copy for Id<T> {}

impl<T: Entity> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T: Entity> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

/// A generic type that can handle any Id
pub struct AnyEntity(Id<AnyEntity>);

impl Entity for AnyEntity {
    fn id(&self) -> Id<AnyEntity> {
        self.0
    }

    fn get(_id: Id<Self>, _context: &HirContext) -> &AnyEntity {
        unreachable!()
    }
}

impl<T: Entity> Id<T> {
    pub fn as_generic(&self) -> Id<AnyEntity> {
        Id(self.0, PhantomData)
    }
}
