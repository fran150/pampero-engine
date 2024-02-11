use std::collections::{HashMap, HashSet};

use uuid::Uuid;

use crate::{entities::{Entities, Entity}, event::Event, game_loop::GameLoopPhase};

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct SystemID(Uuid);

impl SystemID {
    pub fn new() -> Self {
        SystemID(Uuid::new_v4())
    }
}

pub struct Systems<T>(HashMap<SystemID, SystemFunction<T>>);

impl<T> Systems<T> {
    pub fn new() -> Self {
        Systems(HashMap::new())
    }

    pub fn register_system(&mut self, system_function: SystemFunction<T>) -> SystemID {
        let system = SystemID::new();
        self.0.insert(system.clone(), system_function);
        system
    }

    pub fn unregister_system(&mut self, system: &SystemID) {
        self.0.remove(system);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SystemID, &SystemFunction<T>)> {
        self.0.iter()
    }
}

pub struct GameLoopSystems<T> {
    systems: HashMap<GameLoopPhase, Systems<T>>,
}

impl<T> GameLoopSystems<T> {
    pub fn new() -> Self {
        GameLoopSystems {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, game_loop_phase: GameLoopPhase, system_function: SystemFunction<T>) -> SystemID {
        let systems = self.systems.entry(game_loop_phase).or_insert(Systems::new());
        systems.register_system(system_function)
    }

    pub fn unregister_system(&mut self, game_loop_phase: GameLoopPhase, system: &SystemID) {
        if let Some(systems) = self.systems.get_mut(&game_loop_phase) {
            systems.unregister_system(system);
        }
    }

    pub fn call_systems(&self, game_loop_phase: GameLoopPhase, entities: &Entities, components: &mut T, event: &Event) {
        if let Some(systems) = self.systems.get(&game_loop_phase) {
            for (_, system) in systems.iter() {
                for entity in entities.iter() {
                    let context = SystemContext::from(entity.clone(), components, event);
                    system.call(context);
                }
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





pub struct SystemContext<'a, T> {
    pub entity: Entity,
    pub components: &'a mut T,
    pub event: &'a Event
}

impl<'a, T> SystemContext<'a, T> {
    pub fn from(entity: Entity, components: &'a mut T, event: &'a Event) -> Self {
        SystemContext { entity, components, event }
    }
}

pub struct SystemFunction<T>(fn(context: SystemContext<T>) -> Option<()>);

impl<T> SystemFunction<T> {
    pub fn from(funtion: fn(SystemContext<T>) -> Option<()>) -> Self {
        SystemFunction(funtion)
    }

    pub fn call(&self, context: SystemContext<T>) {
        (self.0)(context);
    }
}
