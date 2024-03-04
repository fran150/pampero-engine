use std::collections::HashSet;

use super::Entity;

///
/// Represents a group of [`Entity`] objects. It is normally accessed through the [`ECS`](crate::ecs::ECS)
/// struct or the [`context`](crate::ecs::SystemContext) passed as argument to system functions.
///
/// * See [`crate::ECS::register_system`]
///
/// # Examples
/// ```
/// use pampero_engine::{
///     App,
///     generate_components_struct
/// };
/// 
/// pub struct Test();
/// 
/// generate_components_struct!(
///     Components, 
///     test: Test
/// );
/// 
/// let components = Components::new();
/// let mut app = App::new(components);
///
/// let e1 = app.ecs.spawn_entity();
/// let e2 = app.ecs.spawn_entity();
/// let e3 = app.ecs.spawn_entity();
/// 
/// assert_eq!(true, app.ecs.entities.is_registered(&e1));
/// assert_eq!(true, app.ecs.entities.is_registered(&e2));
/// assert_eq!(true, app.ecs.entities.is_registered(&e3));
/// 
/// app.ecs.remove_entity(&e1);
/// 
/// assert_eq!(false, app.ecs.entities.is_registered(&e1));
/// 
/// let remaining_entities = vec![e2, e3];
/// 
/// let all_exists = app.ecs.entities.iter()
///     .all(|value| { remaining_entities.contains(value) });
/// 
/// assert_eq!(true, all_exists);
/// ```
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


    /// Removes an entity
    pub(crate) fn remove_entity(&mut self, entity: &Entity) -> bool {
        self.0.remove(entity)
    }


    /// Returns if an entity is registered within the ecs system.
    /// # Example
    /// ```
    /// use pampero_engine::{
    ///     App,
    ///     generate_components_struct
    /// };
    /// 
    /// pub struct Name(String);
    /// pub struct Person();
    /// 
    /// generate_components_struct!(
    ///     Components, 
    ///     name: Name,
    ///     person: Person
    /// );
    /// 
    /// let components = Components::new();
    /// let mut app = App::new(components);
    /// 
    /// let person = app.ecs.spawn_entity();
    /// let cat = app.ecs.spawn_entity();
    ///
    /// app.ecs.remove_entity(&cat);
    ///
    /// assert_eq!(true, app.ecs.entities.is_registered(&person));
    /// // Returns false as it was removed from the ecs system
    /// assert_eq!(false, app.ecs.entities.is_registered(&cat));
    /// ```
    pub fn is_registered(&self, entity: &Entity) -> bool {
        self.0.contains(entity)
    }


    /// Returns an iterator over all entities. Useful for filtering entities with specific
    /// attributes
    ///
    /// # Example
    /// ```
    /// use pampero_engine::{
    ///     App,
    ///     generate_components_struct
    /// };
    /// 
    /// pub struct Name(String);
    /// pub struct Person();
    /// 
    /// generate_components_struct!(
    ///     Components, 
    ///     name: Name,
    ///     person: Person
    /// );
    /// 
    /// let components = Components::new();
    /// let mut app = App::new(components);
    /// 
    /// let person = app.ecs.spawn_entity();
    /// let cat = app.ecs.spawn_entity();
    /// let _rock = app.ecs.spawn_entity();
    /// 
    /// app.ecs.components.add_name(&person, Name(String::from("Valen")));
    /// app.ecs.components.add_name(&cat, Name(String::from("Benito")));
    /// app.ecs.components.add_person(&person, Person());
    /// 
    /// // Returns only entities that have name and person components
    /// let only_valen = app.ecs.entities.iter()
    ///     .filter(|entity| {
    ///         app.ecs.components.get_name(entity).is_some() &&
    ///         app.ecs.components.get_person(entity).is_some()
    ///     }).all(|entity| {
    ///         entity == &person
    ///     });
    /// 
    /// assert_eq!(true, only_valen);
    /// 
    pub fn iter(&self) -> impl Iterator<Item = &Entity> {
        self.0.iter()
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_entity_adds_entity() {
        let mut entities = Entities::new();
        let entity = entities.spawn_entity();
        assert_eq!(true, entities.is_registered(&entity));
    }

    #[test]
    fn remove_entity_removes_entity() {
        let mut entities = Entities::new();
        let entity = entities.spawn_entity();
        assert_eq!(true, entities.is_registered(&entity));

        let existed = entities.remove_entity(&entity);
        assert_eq!(false, entities.is_registered(&entity));
        assert_eq!(true, existed);
    }

    #[test]
    fn remove_entity_that_not_exists_returns_false() {
        let mut entities = Entities::new();
        
        let entity = Entity::new();

        let existed = entities.remove_entity(&entity);
        assert_eq!(false, entities.is_registered(&entity));
        assert_eq!(false, existed);
    }

    #[test]
    fn is_registered_returns_correct_value() {
        let mut entities = Entities::new();

        let registered_entity = entities.spawn_entity();
        let unregistered_entity = Entity::new();

        assert_eq!(true, entities.is_registered(&registered_entity));
        assert_eq!(false, entities.is_registered(&unregistered_entity));
    }

    #[test]
    fn iter_iterates_over_all_entities() {
        let mut entities = Entities::new();

        let e1 = entities.spawn_entity();
        let e2 = entities.spawn_entity();

        let created_entities = vec![e1, e2];

        let all_exists = entities.iter()
            .all(|value| { created_entities.contains(value) });

        assert_eq!(true, all_exists);
    }
}