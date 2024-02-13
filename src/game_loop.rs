use std::time::Instant;

use crate::{ecs::EntityDrop, event::{Event, TimeStepEventType}, App};

#[derive(Eq, Hash, PartialEq)]
pub enum GameLoopPhase {
    Init,
    PreLoop,
    PrePhysics,
    Physics,
    PostPhysics,
    Loop,
    PreFrame,
    Frame,
    PostFrame,
    PostLoop,
    Finish,
}

pub struct GameLoop;

impl GameLoop {
    pub fn run<T>(app: &mut App<T>) 
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

        // Init loop

        loop {
            let current_time = instant.elapsed().as_millis();            
            let dt = (current_time - previous_time) as f64;
            previous_time = current_time;
            
            t += dt;
            physics_time += dt;
            frame_time += dt;
            rate_accumulator += dt;

            if physics_time > physics_size {
                physics_time -= physics_size;

                let event = Event::timestep_event(TimeStepEventType::PhysicStep, t, dt);

                app.systems.call_systems(GameLoopPhase::PrePhysics, &event, &mut app.components, &mut app.entities);
                app.systems.call_systems(GameLoopPhase::Physics, &event, &mut app.components, &mut app.entities);
                app.systems.call_systems(GameLoopPhase::PostPhysics, &event, &mut app.components, &mut app.entities);

                physics_step_counter += 1;
            }

            if frame_time > frame_size {
                frame_time -= frame_size;

                let event = Event::timestep_event(TimeStepEventType::FrameStep, t, dt);

                app.systems.call_systems(GameLoopPhase::PreFrame, &event, &mut app.components, &mut app.entities);
                app.systems.call_systems(GameLoopPhase::Frame, &event, &mut app.components, &mut app.entities);
                app.systems.call_systems(GameLoopPhase::PostFrame, &event, &mut app.components, &mut app.entities);
    
                frame_counter += 1;
            }

            if rate_accumulator > 1000.0 {
                println!("{} PPS - {} FPS", physics_step_counter, frame_counter);
                
                physics_step_counter = 0;
                frame_counter = 0;

                rate_accumulator -= 1000.0;
            }
        }
    }
}