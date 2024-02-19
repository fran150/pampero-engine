use core::{GameLoop, GameLoopPhase};

use ecs::{Entities, EntityDrop, Systems, SystemFunction, System};

pub mod ecs;
pub mod core;
pub mod event;

mod macros;

pub struct App<T> {
    entities: Entities,
    components: T,
    systems: Systems<T>,
    run: bool,
}

impl<T> App<T> 
    where T: EntityDrop {
    pub fn new(components: T) -> App<T> {
        App {
            entities: Entities::new(),
            components,
            systems: Systems::new(),
            run: false,
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

    pub fn run(&mut self, game_loop: &mut GameLoop<T>) {
        self.run = true;
        game_loop.run(self);
    }

    pub fn stop(&mut self) {
        self.run = false;
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn run_app() {
    }
}