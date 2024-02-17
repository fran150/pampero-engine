use std::collections::HashMap;

use super::{System, SystemFunction};

pub struct Systems<T>(HashMap<System, SystemFunction<T>>);

impl<T> Systems<T> {
    pub fn new() -> Self {
        Systems(HashMap::new())
    }

    pub fn register_system(&mut self, system_function: SystemFunction<T>) -> System {
        let system = System::new();
        self.0.insert(system.clone(), system_function);
        system
    }

    pub fn unregister_system(&mut self, system: &System) {
        self.0.remove(system);
    }

    pub fn iter(&self) -> impl Iterator<Item = (&System, &SystemFunction<T>)> {
        self.0.iter()
    }
}