use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct System(Uuid);

impl System {
    pub fn new() -> Self {
        System(Uuid::new_v4())
    }
}