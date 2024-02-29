use std::collections::HashMap;

use crate::{events::{Event, GameLoopEventType}, App};

pub struct GameLoopEventHandlers<T> {
    handler: HashMap<GameLoopEventType, fn(&mut App<T>, &Event)>,
}

impl<T> GameLoopEventHandlers<T> {
    pub(crate) fn new() -> Self {
        Self {
            handler: HashMap::new(),
        }
    }

    pub fn set(&mut self, event_type: GameLoopEventType, handler: fn(&mut App<T>, &Event)) {
        self.handler.insert(event_type, handler);
    }

    pub fn clear(&mut self, event_type: GameLoopEventType) {
        self.handler.remove(&event_type);
    }

    pub fn call(&mut self, event_type: GameLoopEventType, app: &mut App<T>, t: f64, dt: f64) {
        if let Some(handler) = self.handler.get(&event_type) {
            let event = Event::game_loop_event(event_type, t, dt);
            handler(app, &event);
        }
    }
}
