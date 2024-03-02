#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopEventPhase {
    Init,
    PreLoop,
    GameLoop,
    PostLoop,
    PrePhysics,
    Physics,
    PostPhysics,
    PreFrame,
    Frame,
    PostFrame,
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

pub enum SystemEvents {
    GameLoopEvent {
        event_type: GameLoopEventPhase,
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
    SystemEvent(SystemEvents),
    UserEvent(),
}

impl Event {
    pub fn game_loop_event(event_type: GameLoopEventPhase, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEvents::GameLoopEvent { event_type, t, dt })
    }

    pub fn timestep_event(event_type: TimeStepEventType, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEvents::TimeStepEvent { event_type, t, dt })
    }

    pub fn keyboard_event(event_type: KeyboardEventType, key: char, t: f64, dt: f64) -> Event {
        Event::SystemEvent(SystemEvents::KeyboardEvent { event_type, key, t, dt })
    }
}