#[macro_export]
macro_rules! generate_components_struct {
    ( $struct_name:ident, $( $component_name:ident: $type:ty ),* ) => {
        paste::paste! {
            pub struct $struct_name {
            $(
                pub [<$component_name _components>]: std::collections::HashMap<$crate::ecs::Entity, $type>,
            )*
            }

            impl $struct_name {
                pub fn new() -> Self {
                    $struct_name {
                        $(
                            [<$component_name _components>]: std::collections::HashMap::new(),
                        )*
                    }
                }

                $(
                    pub fn [<add_ $component_name>](&mut self, entity: &$crate::ecs::Entity, component: $type) {
                        self.[<$component_name _components>].insert(entity.clone(), component);
                    }

                    pub fn [<remove_ $component_name>](&mut self, entity: &$crate::ecs::Entity) {
                        self.[<$component_name _components>].remove(entity);
                    }

                    pub fn [<get_ $component_name>](&self, entity: &$crate::ecs::Entity) -> Option<&$type> {
                        self.[<$component_name _components>].get(entity)
                    } 

                    pub fn [<get_ $component_name _mut>](&mut self, entity: &$crate::ecs::Entity) -> Option<&mut $type> {
                        self.[<$component_name _components>].get_mut(entity)
                    }                                      
                )*
            }

            impl $crate::ecs::EntityDrop for Components {
                fn remove_entity_components(&mut self, entity: &$crate::ecs::Entity) {
                    $(
                        self.[<$component_name _components>].remove(entity);
                    )*
                }
            }
        }
    };
}