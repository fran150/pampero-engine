use std::collections::HashSet;

use super::Entity;

///
/// Represents a group of [`Entity`] objects. It is normally accessed through the [`entities`](crate::ecs::ECS::entities)
/// field of the [`ECS`](crate::ecs::ECS) struct. It can be used to spawn, remove and iterate over
/// all entities.
///
pub struct Entities(HashSet<Entity>);

impl Entities {
    /// Creates a new entity
    pub(crate) fn new() -> Self {
        Self(HashSet::new())
    }

    /// Spawns a new entity and returns it.
    pub(crate) fn spawn_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.0.insert(entity.clone());
        entity        
    }

    /// Removes an entity and all of its components.
    pub(crate) fn remove_entity(&mut self, entity: &Entity) {
        self.0.remove(entity);
    }

    /// Returns if an entity is registered within the ecs system.
    ///
    /// Entities are created by using the [`spawn_entity`](Entities::spawn_entity) function that returns
    /// an [`Entity`] object. This object can be removed from the ECS by using the
    /// [`remove_entity`](Entities::remove_entity) function. This function is useful for checking if an
    /// entity is still registered within the ECS.
    pub fn is_registered(&self, entity: &Entity) -> bool {
        self.0.contains(entity)
    }

    /// Returns an iterator over all entities. Useful for filtering entities with specific
    /// attributes
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn run_app() {
    }
}