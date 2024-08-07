use std::{fmt::Debug, hash::Hash, marker::PhantomData, sync::atomic::AtomicU64};

use crate::{component::Component, AccessComponent, HirContext};

/// Represents what kind of entity this is
///
/// The type of the BaseComponent correlates to this
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EntityKind {
    Root,
    Module,
    File,
    StructDef,
    Field,
    Func,
    TypeAlias,
    Import,
    Global,

    Ty,
    Value,
    Stmt,
    CodeBlock,
    
    Constant,

    SourceFile,

    Local,

    Placeholder,
}

/// An entity is the base data class in `firefly-ecs`
pub struct Entity {
    pub(crate) id: Id<Entity>,
    pub(crate) kind: EntityKind,
    pub(crate) parent: Option<Id<Entity>>,
    pub(crate) children: Vec<Id<Entity>>,
}

pub struct Id<T: Component>(u64, PhantomData<T>);

impl<T: Component> Id<T> {
    pub fn as_base(&self) -> Id<Entity> {
        Id(self.0, PhantomData)
    }

    /// ONLY perform this function with a guarantee
    /// the component will exist
    pub unsafe fn cast<C: Component>(&self) -> Id<C> {
        Id(self.0, PhantomData)
    }
}

impl<T: Component> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Id({})", self.0)
    }
}

impl<T: Component> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: Component> Eq for Id<T> {}

impl<T: Component> Copy for Id<T> {}

impl<T: Component> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0, PhantomData)
    }
}

impl<T: Component> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Component for Entity {}

impl<T: Component> Default for Id<T> {
    fn default() -> Self {
        static BASE: AtomicU64 = AtomicU64::new(0);

        let idx = BASE.fetch_add(1, std::sync::atomic::Ordering::AcqRel);

        Self(idx, PhantomData)
    }
}

impl AccessComponent<Entity> for HirContext {
    fn get_components(&self) -> &std::collections::HashMap<Id<Entity>, Entity> {
        &self.entities
    }

    fn get_components_mut(&mut self) -> &mut std::collections::HashMap<Id<Entity>, Entity> {
        &mut self.entities
    }
}
