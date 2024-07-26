use crate::{items::SourceFile, ComputedComponent, Entity, Id, Root, Visibility};

use super::Symbol;

/// Determines what scope the symbol is visible to
/// 
/// This is determined by a set of rules:
/// - Public symbols are visible in the root
/// - Internal symbols are visible in the base module they are defined in
/// - Fileprivate symbols are visible in the file they are defined in
/// - Private symbols are visible in the immediate parent they are defined in
#[derive(Debug, Clone)]
pub struct VisibleWithin(pub Id<Entity>);

component!(visible_withins: VisibleWithin);

impl ComputedComponent for VisibleWithin {
    fn compute(entity: Id<Entity>, context: &mut crate::HirContext) -> Option<Self> {
        let Symbol { visibility, .. } = context.try_get::<Symbol>(entity)?;

        // Quickly return a value if we can
        match visibility {
            Visibility::Public => return Some(VisibleWithin(context.root().as_base())),
            Visibility::Local => return Some(VisibleWithin(entity)),

            _ => {}
        }

        let mut current = entity;
        while let Some(parent) = context.parent(current) {
            match visibility {
                Visibility::Private => {
                    // Private visibility is only visible to the parent
                    current = parent;
                    break;
                }
                Visibility::FilePrivate => {
                    // Fileprivate visibility is only visible within the file
                    if context.has::<SourceFile>(current) {
                        break
                    }
                }
                Visibility::Internal => {
                    // Internal visibility is visible within the base module
                    if context.has::<Root>(parent) {
                        break
                    }
                }

                Visibility::Public => unreachable!(),
                Visibility::Local => unreachable!(),
            }

            current = parent;
        }

        return Some(VisibleWithin(current))
    }
}