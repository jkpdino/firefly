use std::collections::HashMap;

use crate::{ComputedComponent, Id};

use super::{Namespace, SymbolCollection};

/// The StaticMemberTable provides a list of symbols
/// available from outside the entity. For example,
/// a function will have its parameters and generic parameters
/// in its namespace because they are accessible from inside,
/// but not in it's member table
#[derive(Clone, Debug)]
pub struct StaticMemberTable {
    members: HashMap<String, SymbolCollection>,
}

impl StaticMemberTable {
    pub fn lookup(&self, name: &str) -> Option<SymbolCollection> {
        self.members.get(name).cloned()
    }
}

component!(static_member_tables: StaticMemberTable);

impl ComputedComponent for StaticMemberTable {
    fn compute(entity: Id<crate::Entity>, context: &mut crate::HirContext) -> Option<Self> {
        // A namespace is composed of all the children of an entity which
        // have a symbol. We can filter those elements to get the ones
        // that should be accessible from outside. We don't filter by
        // visibility until we are resolving things.
        let namespace = context.try_get_computed::<Namespace>(entity)?;
        let symbols = namespace.symbols.clone();

        let members = symbols
            .into_iter()
            .map(|id| (id, context.get(id)))
            .filter(|(_, sym)| sym.is_static)
            .fold(
                HashMap::<String, SymbolCollection>::new(),
                |mut map, (id, sym)| {
                    map.entry(sym.name.name.clone()).or_default().add(id);
                    map
                },
            );

        return Some(StaticMemberTable { members });
    }
}

/// The InstanceMemberTable provides a list of symbols
/// available from an instance of an entity. For example,
/// a struct will not have its fields or methods in its namespace
/// or static member table, but it will have them in its instance
/// member table.
#[derive(Clone, Debug)]
pub struct InstanceMemberTable {
    members: HashMap<String, SymbolCollection>,
}

impl InstanceMemberTable {
    pub fn lookup(&self, name: &str) -> Option<SymbolCollection> {
        self.members.get(name).cloned()
    }
}

component!(instance_member_tables: InstanceMemberTable);

impl ComputedComponent for InstanceMemberTable {
    fn compute(entity: Id<crate::Entity>, context: &mut crate::HirContext) -> Option<Self> {
        // A namespace is composed of all the children of an entity which
        // have a symbol. We can filter those elements to get the ones
        // that should be accessible from outside. We don't filter by
        // visibility until we are resolving things.
        let namespace = context.try_get_computed::<Namespace>(entity)?;
        let symbols = namespace.symbols.clone();

        let members = symbols
            .into_iter()
            .map(|id| (id, context.get(id)))
            .filter(|(_, sym)| !sym.is_static)
            .fold(
                HashMap::<String, SymbolCollection>::new(),
                |mut map, (id, sym)| {
                    map.entry(sym.name.name.clone()).or_default().add(id);
                    map
                },
            );

        return Some(InstanceMemberTable { members });
    }
}
