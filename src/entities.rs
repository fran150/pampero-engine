use std::{collections::HashSet, hash::Hash};

use uuid::Uuid;

use crate::App;

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

pub struct EntityDropper(HashSet<Entity>);

impl EntityDropper {
    pub fn new() -> Self {
        Self(HashSet::new())
    }

    pub fn drop_entity(&mut self, entity: &Entity) {
        self.0.insert(entity.clone());
    }

    pub fn drop_all<T>(&mut self, app: &mut App<T>) 
        where T: EntityDrop {
        for entity in self.0.iter() {
            app.entities.remove_entity(entity, &mut app.components);
        }

        self.0.clear();
    }
}