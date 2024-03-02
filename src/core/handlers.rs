use std::collections::HashMap;

use crate::{
    events::{
        Event, 
        GameLoopEventPhase
    }, 
    App
};

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
    handler: HashMap<GameLoopEventPhase, fn(GameLoopContext<T>)>,
}

impl<T> GameLoopEventHandlers<T> {
    pub(crate) fn new() -> Self {
        Self {
            handler: HashMap::new(),
        }
    }

    pub fn set(&mut self, phase: GameLoopEventPhase, handler: fn(GameLoopContext<T>)) {
        self.handler.insert(phase, handler);
    }

    pub fn clear(&mut self, phase: GameLoopEventPhase) {
        self.handler.remove(&phase);
    }

    pub(crate) fn execute(&mut self, phase: GameLoopEventPhase, app: &mut App<T>, t: f64, dt: f64) {
        if let Some(handler) = self.handler.get(&phase) {
            let event = Event::game_loop_event(phase, t, dt);
            let context = GameLoopContext::from(app, &event);

            handler(context);
        }
    }
}
