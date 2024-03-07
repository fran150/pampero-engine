use crate::game_loop::GameLoopPhase;

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
    events: Vec<Event<T>>,
}

impl<T> Events<T> {
    pub(crate) fn new() -> Events<T> {
        Events { events: Vec::new() }
    }

    pub fn dispatch(&mut self, event: Event<T>) {
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<Event<T>> {
        self.events.pop()
    }
}