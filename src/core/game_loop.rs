use std::time::Instant;

use crate::{
    events::GameLoopEventPhase, 
    App
};

use super::handlers::GameLoopEventHandlers;

pub struct GameLoop<T> {
    pub handlers: GameLoopEventHandlers<T>,
}

impl<T> GameLoop<T> {
    pub fn new() -> Self {
        Self {
            handlers: GameLoopEventHandlers::new(),
        }
    }

    pub fn run(&mut self, app: &mut App<T>) {
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

        self.handlers.execute(GameLoopEventPhase::Init, app, t, 0.0);

        while app.run {
            let current_time = instant.elapsed().as_millis();            
            let dt = (current_time - previous_time) as f64;
            previous_time = current_time;
            
            t += dt;
            physics_time += dt;
            frame_time += dt;
            rate_accumulator += dt;

            self.handlers.execute(GameLoopEventPhase::GameLoop, app, t, dt);

            if physics_time > physics_size {
                physics_time -= physics_size;

                self.handlers.execute(GameLoopEventPhase::PrePhysics, app, t, dt);
                self.handlers.execute(GameLoopEventPhase::Physics, app, t, dt);
                self.handlers.execute(GameLoopEventPhase::PostPhysics, app, t, dt);

                physics_step_counter += 1;
            }

            self.handlers.execute(GameLoopEventPhase::GameLoop, app, t, dt);

            if frame_time > frame_size {
                frame_time -= frame_size;

                self.handlers.execute(GameLoopEventPhase::PreFrame, app, t, dt);
                self.handlers.execute(GameLoopEventPhase::Frame, app, t, dt);
                self.handlers.execute(GameLoopEventPhase::PostFrame, app, t, dt);


                frame_counter += 1;
            }

            self.handlers.execute(GameLoopEventPhase::PostLoop, app, t, dt);

            if rate_accumulator > 1000.0 {
                println!("{} PPS - {} FPS", physics_step_counter, frame_counter);
                
                physics_step_counter = 0;
                frame_counter = 0;

                rate_accumulator -= 1000.0;
            }
        }

        self.handlers.execute(GameLoopEventPhase::Finish, app, t, 0.0);
    }
}