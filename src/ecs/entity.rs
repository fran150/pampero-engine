use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Entity(Uuid);

impl Entity {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}