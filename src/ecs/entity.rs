use uuid::Uuid;

///
/// An entity represents a general-purpose object. An entity can be composed by many 
/// `Components`. Each [`System`](crate::ecs::System) will operate in entities with 
/// certain components modifying its state. 
/// Entities are created calling [`crate::ecs::Entities::spawn_entity`].
///
#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

impl Entity {
    /// Creates a new Entity
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

pub trait EntityDrop {
    fn remove_entity_components(&mut self, entity: &Entity);
}