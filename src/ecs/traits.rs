use super::Entity;

pub trait EntityDrop {
    fn remove_entity_components(&mut self, entity: &Entity);
}