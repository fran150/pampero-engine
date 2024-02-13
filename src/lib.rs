use ecs::{Entities, EntityDrop, GameLoopSystems, SystemFunction, System};
use game_loop::GameLoop;

pub mod ecs;

pub use game_loop::GameLoopPhase;
mod macros;
mod game_loop;
pub mod event;

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

    pub fn register_system(&mut self, game_loop_phase: GameLoopPhase, system_function: SystemFunction<T>) -> System {
        self.systems.register_system(game_loop_phase, system_function)
    }

    pub fn unregister_system(&mut self, game_loop_phase: GameLoopPhase, system: &System) {
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