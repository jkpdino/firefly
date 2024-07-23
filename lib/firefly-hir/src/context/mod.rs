mod iter;

use std::collections::HashMap;

use crate::{
    component::{BaseComponent, Component},
    entity::Id,
    func::Func,
    items::{Module, StructDef, TypeAlias},
    resolve::{Namespace, StaticMemberTable, Symbol, SymbolTable},
    ty::HasType,
    util::Root,
    AccessComponent, ComputedComponent, Entity, EntityKind,
};

// The HirContext keeps track of every entity in the system,
// and its related components. It is defined by a macro to simplify
// its component stores
ecs! {
    pub struct HirContext {
        entities: Entity,

        roots: Root,
        funcs: Func,
        modules: Module,
        structs: StructDef,
        typealiases: TypeAlias,

        has_types: HasType,

        symbols: Symbol,
        namespaces: Namespace,
        symbol_tables: SymbolTable,
        static_member_tables: StaticMemberTable
    }
}

impl HirContext {
    /// Adds a new base component to the hir
    ///
    /// Base components have an associated id and
    /// entity kind, so this creates a new entity for them
    /// with that specified kind, and adds a new component
    /// with that type.
    ///
    /// Panics if the entity already exists
    pub fn create<C: BaseComponent>(&mut self, component: C) -> Id<C>
    where
        Self: AccessComponent<C>,
    {
        // Create the new entity, checking if it already exists
        let component_id = component.id();
        let entity_id = component.id().as_base();

        self.ensure_entity_exists(entity_id);
        let Some(entity) = self.entities.get_mut(&entity_id) else {
            unreachable!()
        };

        if entity.kind != EntityKind::Placeholder {
            panic!("tried to create duplicate entities")
        }

        entity.kind = C::ENTITY_KIND;

        // Now add the base component
        let component_map = <Self as AccessComponent<C>>::get_components_mut(self);

        component_map.insert(entity_id, component);

        component_id
    }

    /// Ensures a record for the entity exists, and marks
    /// it as a placeholder
    fn ensure_entity_exists(&mut self, entity_id: Id<Entity>) {
        if self.entities.contains_key(&entity_id) {
            return;
        }

        let new_entity = Entity {
            _id: entity_id,
            kind: EntityKind::Placeholder,
            parent: None,
            children: Vec::new(),
        };
        self.entities.insert(entity_id, new_entity);
    }

    /// Adds a component to an existing (or non existing) entity
    ///
    /// If the component already exists, will replace the original value
    pub fn add_component<C: Component>(&mut self, entity: Id<impl Component>, component: C) -> Id<C>
    where
        Self: AccessComponent<C>,
    {
        let entity_id = entity.as_base();
        let component_map = <Self as AccessComponent<C>>::get_components_mut(self);

        component_map.insert(entity_id, component);

        unsafe { entity_id.cast() }
    }

    /// Returns the component for this entity
    ///
    /// Since we already have an Id for it, we know
    /// that the component exists.
    pub fn get<C: Component>(&self, id: Id<C>) -> &C
    where
        Self: AccessComponent<C>,
    {
        let entity_id = id.as_base();

        let component_map = <Self as AccessComponent<C>>::get_components(self);

        return component_map
            .get(&entity_id)
            .expect("internal compiler error: component doesn't exist");
    }

    /// Returns the specified component for an entity
    /// if it exists.
    ///
    /// There's no way to know at compile time if the
    /// component exists
    pub fn try_get<C: Component>(&self, id: Id<impl Component>) -> Option<&C>
    where
        Self: AccessComponent<C>,
    {
        let entity_id = id.as_base();

        let component_map = <Self as AccessComponent<C>>::get_components(self);

        return component_map.get(&entity_id);
    }

    pub fn try_get_computed<C: ComputedComponent>(&mut self, id: Id<impl Component>) -> Option<&C>
    where
        Self: AccessComponent<C>,
    {
        // todo!: this code is really hacky, but
        // its the only way I could get the borrow tracker to work
        let entity_id = id.as_base();

        if <Self as AccessComponent<C>>::get_components(self).contains_key(&entity_id) {
            return <Self as AccessComponent<C>>::get_components(self).get(&entity_id);
        }

        if let Some(default) = C::compute(entity_id, self) {
            let component_map = <Self as AccessComponent<C>>::get_components_mut(self);
            component_map.insert(entity_id, default);

            return component_map.get(&entity_id);
        }

        None
    }

    /// Casts an id to a different component type
    pub fn cast_id<C: Component>(&self, id: Id<impl Component>) -> Option<Id<C>>
    where
        Self: AccessComponent<C>,
    {
        let entity_id = id.as_base();

        let component_map = <Self as AccessComponent<C>>::get_components(self);

        if !component_map.contains_key(&entity_id) {
            return None;
        }

        Some(unsafe { id.as_base().cast() })
    }

    /// Creates a link between a parent and child entity
    ///
    /// Panics if the child already has a parent
    pub fn link(&mut self, parent_id: Id<impl Component>, child_id: Id<impl Component>) {
        let parent_id = parent_id.as_base();
        let child_id = child_id.as_base();

        self.ensure_entity_exists(parent_id);
        self.ensure_entity_exists(child_id);

        let parent = self
            .entities
            .get_mut(&parent_id)
            .expect("internal compiler error: entity doesn't exist");

        parent.children.push(child_id);

        let child = self
            .entities
            .get_mut(&child_id)
            .expect("internal compiler error: entity doesn't exist");

        if child.parent.replace(parent_id).is_some() {
            panic!("internal compiler error: entity already has a parent");
        }
    }

    /// Searches for all entities with a specific component
    /// and calls the function with the id of the entity.
    pub fn search_for<C: Component>(&mut self, mut f: impl FnMut(Id<C>, &mut HirContext))
        where Self: AccessComponent<C>
    {
        let entities =
        self.entities()
            .filter_map(|entity_id| self.cast_id::<C>(entity_id))
            .collect::<Vec<_>>();

        for entity in entities {
            f(entity, self);
        }
    }

    /// Iterates over all entities in the hir in a breadth-first order
    pub fn entities(&self) -> iter::HirContextEntityIter {
        iter::HirContextEntityIter::new(self)
    }

    /// Returns this entity's parent
    pub fn parent(&self, entity: Id<Entity>) -> Option<Id<Entity>> {
        self.get(entity).parent.clone()
    }

    /// Gets a list of children of the entity
    pub fn children(&self, entity: Id<Entity>) -> &[Id<Entity>] {
        &self.get(entity).children
    }

    /// Returns the root element of the tree
    pub fn root(&self) -> Id<Root> {
        self.root
    }
}
