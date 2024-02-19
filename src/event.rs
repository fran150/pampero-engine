#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopEventType {
    Init,
    PreLoop,
    GameLoop,
    PostLoop,
    Finish
}

#[derive(Eq, Hash, PartialEq)]
pub enum KeyboardEventType {
    KeyPressed,
    KeyReleased,
    KeyHold,
}

#[derive(Eq, Hash, PartialEq)]
pub enum TimeStepEventType {
    PhysicStep,
    FrameStep,
}

pub enum SystemEventType {
    GameLoopEvent {
        event_type: GameLoopEventType,
        t: f64,
        dt: f64,
    },
    KeyboardEvent { 
        event_type: KeyboardEventType,
        key: char, 
        t: f64, 
        dt: f64,
    },
    TimeStepEvent {
        event_type: TimeStepEventType,
        t: f64,
        dt: f64,
    }
}

pub enum Event {
    SystemEvent(SystemEventType),
    UserEvent(),
}

impl Event {
    pub fn game_loop_event(event_type: GameLoopEventType, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEventType::GameLoopEvent { event_type, t, dt })
    }

    pub fn timestep_event(event_type: TimeStepEventType, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEventType::TimeStepEvent { event_type, t, dt })
    }

    pub fn keyboard_event(event_type: KeyboardEventType, key: char, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEventType::KeyboardEvent { event_type, key, t, dt })
    }
}