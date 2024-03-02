use std::collections::HashMap;

use crate::{events::{Event, GameLoopEventType}, App};

pub struct GameLoopContext<'a, T> {
    pub app: &'a mut App<T>, 
    pub event: &'a Event
}

impl<'a, T> GameLoopContext<'a, T> {
    pub(crate) fn from(app: &'a mut App<T>, event: &'a Event) -> Self {
        Self {
            app,
            event
        }
    }
}

pub struct GameLoopEventHandlers<T> {
    handler: HashMap<GameLoopEventType, fn(GameLoopContext<T>)>,
}

impl<T> GameLoopEventHandlers<T> {
    pub(crate) fn new() -> Self {
        Self {
            handler: HashMap::new(),
        }
    }

    pub fn set(&mut self, event_type: GameLoopEventType, handler: fn(GameLoopContext<T>)) {
        self.handler.insert(event_type, handler);
    }

    pub fn clear(&mut self, event_type: GameLoopEventType) {
        self.handler.remove(&event_type);
    }

    pub fn call(&mut self, event_type: GameLoopEventType, app: &mut App<T>, t: f64, dt: f64) {
        if let Some(handler) = self.handler.get(&event_type) {
            let event = Event::game_loop_event(event_type, t, dt);
            let context = GameLoopContext::from(app, &event);
            handler(context);
        }
    }
}
