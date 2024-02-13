#[macro_export]
macro_rules! components_gen {
    ( $( $name:ident: $type:ty ),* ) => {
        paste::paste! {
            pub struct Components {
            $(
                pub [<$name _components>]: std::collections::HashMap<$crate::ecs::Entity, std::cell::RefCell<$type>>,
            )*
            }

            impl Components {
                pub fn new() -> Components {
                    Components {
                        $(
                            [<$name _components>]: std::collections::HashMap::new(),
                        )*
                    }
                }

                $(
                    pub fn [<add_ $name>](&mut self, entity: &$crate::ecs::Entity, component: $type) {
                        self.[<$name _components>].insert(entity.clone(), std::cell::RefCell::new(component));
                    }

                    pub fn [<get_ $name>](&self, entity: &$crate::ecs::Entity) -> Option<&std::cell::RefCell<$type>> {
                        self.[<$name _components>].get(entity)
                    }                
                )*
            }

            impl $crate::ecs::EntityDrop for Components {
                fn remove_entity_components(&mut self, entity: &$crate::ecs::Entity) {
                    $(
                        self.[<$name _components>].remove(entity);
                    )*
                }
            }
        }
    };
}