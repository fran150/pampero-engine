use entities::{Entities, EntityDrop};
use game_loop::GameLoop;
use systems::{GameLoopSystems, SystemFunction, SystemID};

pub use game_loop::GameLoopPhase;
mod macros;
pub mod systems;
mod game_loop;
mod event;
pub mod entities;

pub struct App<T> {
    entities: Entities,
    components: T,
    systems: GameLoopSystems<T>,
}

impl<T> App<T> 
    where T: EntityDrop {
    pub fn new(components: T) -> App<T> {
        App {
            entities: Entities::new(),
            components,
            systems: GameLoopSystems::new(),
        }
    }

    pub fn components(&self) -> &T {
        &self.components
    }

    pub fn components_mut(&mut self) -> &mut T {
        &mut self.components
    }

    pub fn entities(&self) -> &Entities {
        &self.entities
    }

    pub fn entities_mut(&mut self) -> &mut Entities {
        &mut self.entities
    }

    pub fn register_system(&mut self, game_loop_phase: GameLoopPhase, system_function: SystemFunction<T>) -> SystemID {
        self.systems.register_system(game_loop_phase, system_function)
    }

    pub fn unregister_system(&mut self, game_loop_phase: GameLoopPhase, system: &SystemID) {
        self.systems.unregister_system(game_loop_phase, system);
    }

    pub fn run(&mut self) {
        GameLoop::run(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_app() {
    }
}