use std::collections::HashMap;

use super::{Entities, System, SystemContext };

use crate::events::Event;

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

    pub fn call_systems(&self, system_group: String, event: &Event, components: &mut T, entities: &mut Entities) {
        if let Some(systems) = self.systems.get(&system_group) {
            for (_, system) in systems.iter() {
                let context = SystemContext::from(event, components, entities);
                system(context);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &HashMap<System, fn(SystemContext<T>)>)> {
        self.systems.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&String, &mut HashMap<System, fn(SystemContext<T>)>)> {
        self.systems.iter_mut()
    }

    pub fn systems(&self, system_group: String) -> Option<&HashMap<System, fn(SystemContext<T>)>> {
        self.systems.get(&system_group)
    }

    pub fn systems_mut(&mut self, system_group: String) -> Option<&mut HashMap<System, fn(SystemContext<T>)>> {
        self.systems.get_mut(&system_group)
    }
}