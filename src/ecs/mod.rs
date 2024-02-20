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

use crate::core::GameLoopPhase;
use crate::event::Event;

pub struct ECS<T> {
    entities: Entities,
    components: T,
    systems: Systems<T>,
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

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn components(&self) -> &T {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut T {
        &mut self.components
    }

    pub fn systems(&self) -> &Systems<T> {
        &self.systems
    }

    pub fn systems_mut(&mut self) -> &mut Systems<T> {
        &mut self.systems
    }

    pub fn call_systems(&mut self, game_loop_phase: GameLoopPhase, event: &Event) {
        self.systems.call_systems(game_loop_phase, event, &mut self.components, &mut self.entities);
    }
}