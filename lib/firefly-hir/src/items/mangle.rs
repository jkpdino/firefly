use firefly_mangle::SymbolName;

use crate::{resolve::Symbol, ComputedComponent, Entity, EntityKind, HirContext, Id};

#[derive(Debug, Clone)]
pub struct MangledName {
    pub symbol: SymbolName
}

component!(mangled_names: MangledName);

impl ComputedComponent for MangledName {
    fn compute(id: Id<Entity>, context: &mut HirContext) -> Option<Self> {
        let mut ancestors = Vec::new();
        let mut current = Some(id);

        while let Some(id) = current {
            if let Some(Symbol { name, .. }) = context.try_get(id) {
                ancestors.push(name.name.clone());
            };

            current = context.parent(id);
        }

        ancestors.reverse();
        

        let path = firefly_mangle::Path::new(ancestors);

        let entity = context.get(id);
        
        let symbol = match entity.kind {
            EntityKind::StructDef => SymbolName::Struct(path),
            EntityKind::Func => SymbolName::Func(path),
            EntityKind::Global => SymbolName::Var(path),

            _ => return None,
        };

        return Some(MangledName { symbol })
    }
}