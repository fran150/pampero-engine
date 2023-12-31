use std::collections::HashSet;
use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

mod macros;

pub struct App<T> {
    entities: HashSet<Entity>,
    components: T,
    systems: HashSet<fn(Entity, &mut T) -> Option<()>>,
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
            systems: HashSet::new(),
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

    pub fn register_system(&mut self, system: fn(Entity, &mut T) -> Option<()>) -> bool {
        self.systems.insert(system)
    }

    pub fn unregister_system(&mut self, system: &fn(Entity, &mut T) -> Option<()>) -> bool {
        self.systems.remove(system)
    }

    pub fn run(&mut self) {
        for entity in self.entities.iter() {
            for system in self.systems.iter() {
                system(entity.clone(), &mut self.components);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_app() {
    }
}