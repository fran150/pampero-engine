use std::collections::LinkedList;

use crate::{
    ecs::EntityDrop, 
    game_loop::{
        GameLoopHandlerContext, 
        GameLoopPhase
    }
};

#[derive(Eq, Hash, PartialEq)]
pub enum KeyboardEventType {
    KeyPressed,
    KeyReleased,
    KeyHold,
}

pub enum SystemEvents {
    GameLoopEvent {
        phase: GameLoopPhase,
        t: f64,
        dt: f64,
    },
    KeyboardEvent { 
        event_type: KeyboardEventType,
        key: char, 
        t: f64, 
        dt: f64,
    }
}

pub enum Event<T> {
    SystemEvent(SystemEvents),
    UserEvent(T),
}

impl<T> Event<T> {
    pub(crate) fn game_loop_event(phase: GameLoopPhase, t: f64, dt: f64) -> Event<T> {
        Event::SystemEvent(SystemEvents::GameLoopEvent { phase, t, dt })
    }

    // Should be pub(crate) or might be replaced by exposing SDL events
    pub fn keyboard_event(event_type: KeyboardEventType, key: char, t: f64, dt: f64) -> Event<T> {
        Event::SystemEvent(SystemEvents::KeyboardEvent { event_type, key, t, dt })
    }

    pub fn user_event(event: T) -> Event<T> {
        Event::UserEvent(event)
    }
}

pub struct Events<T> {
    events: LinkedList<Event<T>>,
}

impl<T> Events<T> {
    pub(crate) fn new() -> Events<T> {
        Events { events: LinkedList::new() }
    }

    pub fn trigger(&mut self, event: Event<T>) {
        self.events.push_back(event);
    }

    pub fn pop(&mut self) -> Option<Event<T>> {
        self.events.pop_front()
    }

    pub fn peek(&self) -> Option<&Event<T>> {
        self.events.front()
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }
}

pub struct EventDispatcher;

impl EventDispatcher {
    pub fn dispatch<T: EntityDrop, U, F: Fn(&Event<U>) -> Option<String>>(mapper: F, context: GameLoopHandlerContext<T, U>) {
        while let Some(event) = context.app.events.pop() {
            if let Some(group) = mapper(&event) {
                context.app.ecs.run_systems(group, &event);
            } else {
                context.app.events.trigger(event);
            }
        }
    }
}