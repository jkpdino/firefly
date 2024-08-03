use std::{fmt::Display, marker::PhantomData, ops::{Deref, DerefMut}};

use crate::ir::VirContext;

pub struct Id<T>(usize, PhantomData<T>);

pub struct IdFactory<T>(usize, PhantomData<T>);

pub struct Container<T> {
    items:   Vec<T>,
    factory: IdFactory<T>,
}

pub type UniqueId<T> = Id<T>;
pub type UniqueIdFactory<T> = IdFactory<T>;
pub type UniqueContainer<T> = Container<T>;

impl<T> Id<T> {
    pub fn new(index: usize) -> Self {
        Id(index, PhantomData)
    }

    pub(crate) fn index(&self) -> usize {
        self.0
    }
}

impl<T> IdFactory<T> {
    pub fn new() -> Self {
        Self(0, PhantomData)
    }

    pub fn next(&mut self) -> Id<T> {
        let num = self.0;

        self.0 += 1;

        Id(num, PhantomData)
    }
}

impl<T> Container<T> {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            factory: IdFactory::new(),
        }
    }

    pub fn next(&mut self) -> Id<T> {
        self.factory.next()
    }

    pub fn get_by_id(&self, id: Id<T>) -> Option<&T> {
        self.get(id.0)
    }

    pub fn get_mut_by_id(&mut self, id: Id<T>) -> Option<&mut T> {
        self.get_mut(id.0)
    }
}

impl<T> Deref for Container<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<T> DerefMut for Container<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone(), self.1.clone())
    }
}

impl<T> Copy for Id<T> { }

pub trait DisplayInContext {
    fn fmt(&self, f: &mut std::fmt::Formatter, context: &VirContext) -> std::fmt::Result;
}

pub struct InContext<'a, T> {
    context: &'a VirContext,
    value:   &'a T
}

impl VirContext {
    pub fn display<'a, T: DisplayInContext>(&'a self, value: &'a T) -> InContext<'a, T> {
        InContext {
            context: self,
            value: value
        }
    }
}

impl<'a, T> Display for InContext<'a, T>
    where T: DisplayInContext
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f, self.context)
    }
}