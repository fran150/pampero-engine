use uuid::Uuid;

///
/// An entity represents a general-purpose object. An entity can be composed by many 
/// `Components`. Each [`System`](crate::ecs::System) will operate in entities with 
/// certain components modifying its state. 
/// Entities are created calling [`crate::ecs::Entities::spawn_entity`].
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
/// ```

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

impl Entity {
    /// Creates a new Entity
    pub(crate) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}