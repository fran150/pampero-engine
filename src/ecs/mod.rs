use crate::events::Event;

mod entity;
mod entities;
mod system;
mod systems;

pub use entity::Entity;
pub use entity::EntityDrop;
pub use entities::Entities;

pub use system::System;
pub use systems::SystemContext;
pub use systems::Systems;

#[non_exhaustive]
pub struct ECS<T, U> {
    pub entities: Entities,
    pub components: T,
    pub systems: Systems<T, U>,
}

impl<T, U> ECS<T, U> 
    where T: EntityDrop {
    pub fn new(components: T) -> Self {
        Self {
            entities: Entities::new(),
            components,
            systems: Systems::new(),
        }
    }

    pub fn spawn_entity(&mut self) -> Entity {
        self.entities.spawn_entity()
    }

    /// Removes an entity and all of its components.
    pub fn remove_entity(&mut self, entity: &Entity) 
        where T: EntityDrop {
        self.components.remove_entity_components(entity);
        self.entities.remove_entity(entity);
    }

    pub fn register_system(&mut self, system_group: String, system_function: fn(SystemContext<T, U>)) -> System {
        self.systems.register_system(system_group, system_function)
    }

    pub fn unregister_system(&mut self, system_group: String, system: &System) {
        self.systems.unregister_system(system_group, system)
    }

    pub fn run_systems(&mut self, group: String, event: &Event<U>) {
        self.systems.call(group, event, &mut self.components, &mut self.entities);
    }
}