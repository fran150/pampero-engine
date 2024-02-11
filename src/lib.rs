use std::collections::HashSet;
use game_loop::GameLoop;
use systems::{GameLoopSystems, SystemFunction, SystemID};
use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

pub use game_loop::GameLoopPhase;
mod macros;
pub mod systems;
mod game_loop;
mod event;

pub struct App<T> {
    entities: HashSet<Entity>,
    components: T,
    systems: GameLoopSystems<T>,
}

pub trait EntityDrop {
    fn remove_entity_components(&mut self, entity: &Entity);
}

impl<T> App<T> 
    where T: EntityDrop {
    pub fn new(components: T) -> App<T> {
        App {
            entities: HashSet::new(),
            components,
            systems: GameLoopSystems::new(),
        }
    }

    pub fn components(&mut self) -> &mut T {
        &mut self.components
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let entity = Entity(Uuid::new_v4());
        self.entities.insert(entity.clone());
        entity        
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.components().remove_entity_components(entity);
        self.entities.remove(entity);
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