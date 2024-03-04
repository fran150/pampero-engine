use uuid::Uuid;

///
/// An entity represents a general-purpose object. It can be composed by many 
/// `Components`. Each [`System`](crate::ecs::System) can be used to operate in entities with 
/// certain components modifying its state. 
///
/// * See [`spawn_entity`](crate::ecs::ECS::spawn_entity)
/// * Also see [`spawn_entity`](crate::ecs::ECS::remove_entity)
///
#[derive(Eq, Hash, PartialEq, Clone)]
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