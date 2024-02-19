use std::collections::HashSet;

use super::{Entity, EntityDrop};

pub struct Entities(HashSet<Entity>);

impl Entities {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.0.insert(entity.clone());
        entity        
    }

    pub fn remove_entity<T>(&mut self, entity: &Entity, components: &mut T) 
        where T: EntityDrop {
        components.remove_entity_components(entity);
        self.0.remove(entity);
    }

    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}