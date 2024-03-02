use std::collections::HashSet;

use super::{
    Entity, 
    EntityDrop
};

///
/// Represents a group of [`Entity`] objects. It is normally accessed through the [`entities`](crate::ecs::ECS::entities)
/// field of the [`ECS`](crate::ecs::ECS) struct. It can be used to spawn, remove and iterate over
/// all entities.
///
/// # Example
/// ```
/// use pampero_engine::App;
/// use pampero_engine::ecs::ECS;
/// 
/// use pampero_engine::components_gen;
///
/// pub struct Person();
///
/// pub struct Name(String);
///
/// components_gen!(
///    person: Person, 
///    name: Name
/// );
///
/// let mut app = App::new();
/// let components = Components::new();
/// let mut ecs = ECS::new(components);
///
/// // Spawns an entity
/// let valen = ecs.entities.spawn_entity();
///
/// // "Valen" entity is a person and has a name
/// ecs.components.add_person(&valen, Person());
/// ecs.components.add_name(&valen, Name("Valen".to_string()));
///
/// // Prints all entities that are persons and have a name
/// ecs.entities.iter().filter(|e| {
///     ecs.components.get_name(e).is_some() && 
///     ecs.components.get_person(e).is_some()
/// }).for_each(|entity| {
///    let name = ecs.components.get_name(entity).unwrap();
///    
///    println!("Person: {}", name.borrow().0);
/// });
///
/// ```
pub struct Entities(HashSet<Entity>);

impl Entities {
    /// Creates a new entity
    pub(crate) fn new() -> Self {
        Self(HashSet::new())
    }

    /// Spawns a new entity and returns it.
    pub fn spawn_entity(&mut self) -> Entity {
        let entity = Entity::new();
        self.0.insert(entity.clone());
        entity        
    }

    /// Removes an entity and all of its components.
    pub fn remove_entity<T>(&mut self, entity: &Entity, components: &mut T) 
        where T: EntityDrop {
        components.remove_entity_components(entity);
        self.0.remove(entity);
    }

    /// Returns an iterator over all entities. Useful for filtering entities with specific
    /// attributes
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}