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

pub struct SystemFunction<T>(fn(context: SystemContext<T>));

impl<T> SystemFunction<T> {
    pub fn from(funtion: fn(SystemContext<T>)) -> Self {
        SystemFunction(funtion)
    }

    pub fn call(&self, context: SystemContext<T>) {
        (self.0)(context);
    }
}