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
pub struct ECS<T> {
    pub entities: Entities,
    pub components: T,
    pub systems: Systems<T>,
}

impl<T> ECS<T> 
    where T: EntityDrop {
    pub fn new(components: T) -> Self {
        Self {
            entities: Entities::new(),
            components,
            systems: Systems::new(),
        }
    }

    pub fn run_systems(&mut self, group: String, event: &Event) {
        self.systems.call(group, event, &mut self.components, &mut self.entities);
    }
}