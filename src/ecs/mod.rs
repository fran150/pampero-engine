mod entity;
mod entities;
mod system;
mod functions;
mod traits;
mod systems;

pub use entity::Entity;
pub use traits::EntityDrop;
pub use entities::Entities;

pub use system::System;
pub use functions::SystemContext;
pub use functions::SystemFunction;

pub use systems::Systems;

use crate::core::GameLoopStep;
use crate::event::Event;

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

    pub fn call_systems(&mut self, game_loop_phase: GameLoopStep, event: &Event) {
        self.systems.call_systems(game_loop_phase, event, &mut self.components, &mut self.entities);
    }
}