use crate::events::Event;

use super::Entities;

pub struct SystemContext<'a, T> {
    pub event: &'a Event,
    pub components: &'a mut T,
    pub entities: &'a mut Entities,
}

impl<'a, T> SystemContext<'a, T> {
    pub(crate) fn from(event: &'a Event, components: &'a mut T, entities: &'a mut Entities) -> Self {
        SystemContext { event, components, entities }
    }
}