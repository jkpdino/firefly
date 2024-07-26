use std::collections::HashMap;

use crate::{ComputedComponent, Id};

use super::{Namespace, Symbol};

/// The StaticMemberTable provides a list of symbols
/// available from outside the entity. For example,
/// a function will have its parameters and generic parameters
/// in its namespace because they are accessible from inside,
/// but not in it's member table
#[derive(Clone, Debug)]
pub struct StaticMemberTable {
    members: HashMap<String, Id<Symbol>>,
}

impl StaticMemberTable {
    pub fn lookup(&self, name: &str) -> Option<Id<Symbol>> {
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

        let members = symbols.into_iter()
            .map(|id| (id, context.get(id)))
            .filter(|_| true)
            .map(|(id, sym)| (sym.name.name.clone(), id))
            .collect::<HashMap<_, _>>();

        return Some(StaticMemberTable { members });
    }
}
