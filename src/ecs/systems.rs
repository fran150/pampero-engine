use std::collections::HashMap;

use super::{
    Entities, 
    System 
};

use crate::events::Event;

pub struct SystemContext<'a, T, U> {
    pub event: &'a Event<U>,
    pub components: &'a mut T,
    pub entities: &'a mut Entities,
}

impl<'a, T, U> SystemContext<'a, T, U> {
    pub(crate) fn from(event: &'a Event<U>, components: &'a mut T, entities: &'a mut Entities) -> Self {
        SystemContext { event, components, entities }
    }
}

pub struct Systems<T, U> {
    systems: HashMap<String, HashMap<System, fn(SystemContext<T, U>)>>,
}

impl<T, U> Systems<T, U> {
    pub(crate) fn new() -> Self {
        Systems {
            systems: HashMap::new(),
        }
    }

    pub(crate) fn register_system(&mut self, system_group: String, system_function: fn(SystemContext<T, U>)) -> System {
        let systems = self.systems.entry(system_group).or_insert(HashMap::new());
        let system = System::new();
        systems.insert(system.clone(), system_function);
        system
    }

    pub(crate) fn unregister_system(&mut self, system_group: String, system: &System) {
        if let Some(systems) = self.systems.get_mut(&system_group) {
            systems.remove(system);
        }
    }

    pub(crate) fn call(&self, system_group: String, event: &Event<U>, components: &mut T, entities: &mut Entities) {
        if let Some(systems) = self.systems.get(&system_group) {
            for (_, system) in systems.iter() {
                let context = SystemContext::from(event, components, entities);
                system(context);
            }
        }
    }
}