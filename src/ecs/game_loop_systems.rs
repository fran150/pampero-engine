use std::collections::HashMap;

use crate::core::GameLoopPhase;

use super::{Entities, System, SystemContext, SystemFunction, Systems };

use crate::event::Event;

pub struct GameLoopSystems<T> {
    systems: HashMap<GameLoopPhase, Systems<T>>,
}

impl<T> GameLoopSystems<T> {
    pub fn new() -> Self {
        GameLoopSystems {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, game_loop_phase: GameLoopPhase, system_function: SystemFunction<T>) -> System {
        let systems = self.systems.entry(game_loop_phase).or_insert(Systems::new());
        systems.register_system(system_function)
    }

    pub fn unregister_system(&mut self, game_loop_phase: GameLoopPhase, system: &System) {
        if let Some(systems) = self.systems.get_mut(&game_loop_phase) {
            systems.unregister_system(system);
        }
    }

    pub fn call_systems(&self, game_loop_phase: GameLoopPhase, event: &Event, components: &mut T, entities: &mut Entities) {
        if let Some(systems) = self.systems.get(&game_loop_phase) {
            for (_, system) in systems.iter() {
                let context = SystemContext::from(event, components, entities);
                system.call(context);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GameLoopPhase, &Systems<T>)> {
        self.systems.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&GameLoopPhase, &mut Systems<T>)> {
        self.systems.iter_mut()
    }

    pub fn systems(&self, game_loop_phase: GameLoopPhase) -> Option<&Systems<T>> {
        self.systems.get(&game_loop_phase)
    }

    pub fn systems_mut(&mut self, game_loop_phase: GameLoopPhase) -> Option<&mut Systems<T>> {
        self.systems.get_mut(&game_loop_phase)
    }
}