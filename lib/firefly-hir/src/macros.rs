macro_rules! component {
    (base($kind:expr) $name:ident : $component:ty) => {
        impl crate::Component for $component {}

        impl crate::BaseComponent for $component {
            const ENTITY_KIND: EntityKind = $kind;

            fn id(&self) -> Id<Self> {
                self.id
            }
        }

        impl crate::AccessComponent<$component> for crate::HirContext {
            fn get_components(&self) -> &std::collections::HashMap<Id<crate::Entity>, $component> {
                &self.$name
            }

            fn get_components_mut(
                &mut self,
            ) -> &mut std::collections::HashMap<Id<crate::Entity>, $component> {
                &mut self.$name
            }
        }
    };

    ($name:ident : $component:ty) => {
        impl crate::Component for $component {}

        impl crate::AccessComponent<$component> for crate::HirContext {
            fn get_components(
                &self,
            ) -> &std::collections::HashMap<crate::Id<crate::Entity>, $component> {
                &self.$name
            }

            fn get_components_mut(
                &mut self,
            ) -> &mut std::collections::HashMap<crate::Id<crate::Entity>, $component> {
                &mut self.$name
            }
        }
    };
}

macro_rules! ecs {
    (
        $v:vis struct $ecs_name:ident {
            $($name:ident : $component:ty),*
        }
    ) => {
        $v struct $ecs_name {
            root: Id<Root>,
        $(
            pub(crate) $name: HashMap<Id<Entity>, $component>
        ),*
        }

        impl $ecs_name {
            pub fn new() -> Self {
                let root = Root::default();

                let mut context = HirContext {
                    root: root.id(),

                    $(
                        $name: HashMap::new()
                    ),*
                };

                context.create(root);

                return context;
            }
        }
    };
}
