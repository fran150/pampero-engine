use std::collections::HashMap;

use crate::{
    events::Event, 
    App
};

use super::GameLoopPhase;

pub struct GameLoopHandlerContext<'a, T> {
    pub app: &'a mut App<T>, 
    pub event: &'a Event
}

impl<'a, T> GameLoopHandlerContext<'a, T> {
    pub(crate) fn from(app: &'a mut App<T>, event: &'a Event) -> Self {
        Self {
            app,
            event
        }
    }
}

pub struct GameLoopHandlers<T> {
    handler: HashMap<GameLoopPhase, fn(GameLoopHandlerContext<T>)>,
}

impl<T> GameLoopHandlers<T> {
    pub(crate) fn new() -> Self {
        Self {
            handler: HashMap::new(),
        }
    }

    pub fn set(&mut self, phase: GameLoopPhase, handler: fn(GameLoopHandlerContext<T>)) {
        self.handler.insert(phase, handler);
    }

    pub fn clear(&mut self, phase: GameLoopPhase) {
        self.handler.remove(&phase);
    }

    pub(crate) fn execute(&mut self, phase: GameLoopPhase, app: &mut App<T>, t: f64, dt: f64) {
        if let Some(handler) = self.handler.get(&phase) {
            let event = Event::game_loop_event(phase, t, dt);
            let context = GameLoopHandlerContext::from(app, &event);

            handler(context);
        }
    }
}
