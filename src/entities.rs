use std::{collections::HashSet, hash::Hash};

use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

pub struct Entities(HashSet<Entity>);

pub trait EntityDrop {
    fn remove_entity_components(&mut self, entity: &Entity);
}

impl Entities {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn spawn_entity(&mut self) -> Entity {
        let entity = Entity(Uuid::new_v4());
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