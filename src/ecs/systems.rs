use std::collections::HashMap;

use crate::core::GameLoopStep;

use super::{Entities, System, SystemContext, SystemFunction };

use crate::event::Event;

pub struct Systems<T> {
    systems: HashMap<GameLoopStep, HashMap<System, SystemFunction<T>>>,
}

impl<T> Systems<T> {
    pub(crate) fn new() -> Self {
        Systems {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, game_loop_phase: GameLoopStep, system_function: SystemFunction<T>) -> System {
        let systems = self.systems.entry(game_loop_phase).or_insert(HashMap::new());
        let system = System::new();
        systems.insert(system.clone(), system_function);
        system
    }

    pub fn unregister_system(&mut self, game_loop_phase: GameLoopStep, system: &System) {
        if let Some(systems) = self.systems.get_mut(&game_loop_phase) {
            systems.remove(system);
        }
    }

    pub fn call_systems(&self, game_loop_phase: GameLoopStep, event: &Event, components: &mut T, entities: &mut Entities) {
        if let Some(systems) = self.systems.get(&game_loop_phase) {
            for (_, system) in systems.iter() {
                let context = SystemContext::from(event, components, entities);
                system.call(context);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&GameLoopStep, &HashMap<System, SystemFunction<T>>)> {
        self.systems.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&GameLoopStep, &mut HashMap<System, SystemFunction<T>>)> {
        self.systems.iter_mut()
    }

    pub fn systems(&self, game_loop_phase: GameLoopStep) -> Option<&HashMap<System, SystemFunction<T>>> {
        self.systems.get(&game_loop_phase)
    }

    pub fn systems_mut(&mut self, game_loop_phase: GameLoopStep) -> Option<&mut HashMap<System, SystemFunction<T>>> {
        self.systems.get_mut(&game_loop_phase)
    }
}