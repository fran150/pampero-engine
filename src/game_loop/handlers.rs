use std::collections::HashMap;

use crate::{
    events::Event, 
    App
};

use super::GameLoopPhase;

pub struct GameLoopHandlerContext<'a, T, U> {
    pub app: &'a mut App<T, U>, 
    pub event: &'a Event<U>
}

impl<'a, T, U> GameLoopHandlerContext<'a, T, U> {
    pub(crate) fn from(app: &'a mut App<T, U>, event: &'a Event<U>) -> Self {
        Self {
            app,
            event
        }
    }
}

pub struct GameLoopHandlers<T, U> {
    handler: HashMap<GameLoopPhase, fn(GameLoopHandlerContext<T, U>)>,
}

impl<T, U> GameLoopHandlers<T, U> {
    pub(crate) fn new() -> Self {
        Self {
            handler: HashMap::new(),
        }
    }

    pub fn set(&mut self, phase: GameLoopPhase, handler: fn(GameLoopHandlerContext<T, U>)) {
        self.handler.insert(phase, handler);
    }

    pub fn clear(&mut self, phase: GameLoopPhase) {
        self.handler.remove(&phase);
    }

    pub(crate) fn execute(&mut self, phase: GameLoopPhase, app: &mut App<T, U>, t: f64, dt: f64) {
        if let Some(handler) = self.handler.get(&phase) {
            let event = Event::game_loop_event(phase, t, dt);
            let context = GameLoopHandlerContext::from(app, &event);

            handler(context);
        }
    }
}
