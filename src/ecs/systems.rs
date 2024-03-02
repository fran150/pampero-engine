use std::collections::HashMap;

use super::{
    Entities, 
    System 
};

use crate::events::Event;

pub struct SystemContext<'a, T> {
    pub event: &'a Event,
    pub components: &'a mut T,
    pub entities: &'a mut Entities,
}

impl<'a, T> SystemContext<'a, T> {
    pub(crate) fn from(event: &'a Event, components: &'a mut T, entities: &'a mut Entities) -> Self {
        SystemContext { event, components, entities }
    }
}

pub struct Systems<T> {
    systems: HashMap<String, HashMap<System, fn(SystemContext<T>)>>,
}

impl<T> Systems<T> {
    pub(crate) fn new() -> Self {
        Systems {
            systems: HashMap::new(),
        }
    }

    pub fn register_system(&mut self, system_group: String, system_function: fn(SystemContext<T>)) -> System {
        let systems = self.systems.entry(system_group).or_insert(HashMap::new());
        let system = System::new();
        systems.insert(system.clone(), system_function);
        system
    }

    pub fn unregister_system(&mut self, system_group: String, system: &System) {
        if let Some(systems) = self.systems.get_mut(&system_group) {
            systems.remove(system);
        }
    }

    pub(crate) fn call(&self, system_group: String, event: &Event, components: &mut T, entities: &mut Entities) {
        if let Some(systems) = self.systems.get(&system_group) {
            for (_, system) in systems.iter() {
                let context = SystemContext::from(event, components, entities);
                system(context);
            }
        }
    }
}