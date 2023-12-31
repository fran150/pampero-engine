
#[macro_export]
macro_rules! components_gen {
    ( $( $name:ident: $type:ty ),* ) => {
        paste! {
            pub struct Components {
            $(
                pub [<$name _components>]: HashMap<Entity, RefCell<$type>>,
            )*
            }

            impl Components {
                pub fn new() -> Components {
                    Components {
                        $(
                            [<$name _components>]: HashMap::new(),
                        )*
                    }
                }

                $(
                    pub fn [<add_ $name>](&mut self, entity: &Entity, component: $type) {
                        self.[<$name _components>].insert(entity.clone(), RefCell::new(component));
                    }

                    pub fn [<get_ $name>](&self, entity: &Entity) -> Option<&RefCell<$type>> {
                        self.[<$name _components>].get(entity)
                    }                
                )*
            }

            impl EntityDrop for Components {
                fn remove_entity_components(&mut self, entity: &Entity) {
                    $(
                        self.[<$name _components>].remove(entity);
                    )*
                }
            }
        }
    };
}