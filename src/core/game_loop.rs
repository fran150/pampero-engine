use std::time::Instant;

use crate::{ecs::{ECS, EntityDrop}, event::{Event, GameLoopEventType, TimeStepEventType}, App};

use super::handlers::GameLoopEventHandlers;

#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopPhase {
    PrePhysics,
    Physics,
    PostPhysics,
    PreFrame,
    Frame,
    PostFrame,
}

pub struct GameLoop<T> {
    handlers: GameLoopEventHandlers<T>,
}

impl<T> GameLoop<T> {
    pub fn new() -> Self {
        Self {
            handlers: GameLoopEventHandlers::new(),
        }
    }

    pub fn handlers(&self) -> &GameLoopEventHandlers<T> {
        &self.handlers
    }

    pub fn handlers_mut(&mut self) -> &mut GameLoopEventHandlers<T> {
        &mut self.handlers
    }

    pub fn run(&mut self, app: &mut App, ecs: &mut ECS<T>) 
        where T: EntityDrop {
        const PPS:f64 = 60.0;
        const FPS:f64 = 60.0;
        
        let instant = Instant::now();
        let mut t = 0.0;

        let mut previous_time = 0;
        let mut physics_time = 0.0;
        let mut frame_time = 0.0;

        let mut rate_accumulator = 0.0;
        let mut physics_step_counter = 0;
        let mut frame_counter = 0;

        let physics_size = 1000.0 / PPS;
        let frame_size = 1000.0 / FPS;

        self.handlers.call(GameLoopEventType::Init, app, ecs, t, 0.0);

        while app.run {
            let current_time = instant.elapsed().as_millis();            
            let dt = (current_time - previous_time) as f64;
            previous_time = current_time;
            
            t += dt;
            physics_time += dt;
            frame_time += dt;
            rate_accumulator += dt;

            self.handlers.call(GameLoopEventType::GameLoop, app, ecs, t, dt);

            if physics_time > physics_size {
                physics_time -= physics_size;

                let event = Event::timestep_event(TimeStepEventType::PhysicStep, t, dt);

                ecs.call_systems(GameLoopPhase::PrePhysics, &event);
                ecs.call_systems(GameLoopPhase::Physics, &event);
                ecs.call_systems(GameLoopPhase::PostPhysics, &event);

                physics_step_counter += 1;
            }

            self.handlers.call(GameLoopEventType::GameLoop, app, ecs, t, dt);

            if frame_time > frame_size {
                frame_time -= frame_size;

                let event = Event::timestep_event(TimeStepEventType::FrameStep, t, dt);

                ecs.call_systems(GameLoopPhase::PreFrame, &event);
                ecs.call_systems(GameLoopPhase::Frame, &event);
                ecs.call_systems(GameLoopPhase::PostFrame, &event);

                frame_counter += 1;
            }

            self.handlers.call(GameLoopEventType::PostLoop, app, ecs, t, dt);

            if rate_accumulator > 1000.0 {
                println!("{} PPS - {} FPS", physics_step_counter, frame_counter);
                
                physics_step_counter = 0;
                frame_counter = 0;

                rate_accumulator -= 1000.0;
            }
        }

        self.handlers.call(GameLoopEventType::Finish, app, ecs, t, 0.0);
    }
}