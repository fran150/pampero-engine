use core::GameLoop;

use ecs::{ECS, EntityDrop};

pub mod ecs;
pub mod core;
pub mod events;

mod macros;

pub struct App<T> {
    run: bool,
    pub ecs: ECS<T>
}

impl<T> App<T> {
    
    pub fn new(components: T) -> Self
        where T: EntityDrop {
        App {
            ecs: ECS::new(components),
            run: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.run
    }

    pub fn run(&mut self, game_loop: &mut GameLoop<T>) 
        where T: EntityDrop {
        self.run = true;
        game_loop.run(self);
    }

    pub fn stop(&mut self) {
        self.run = false;
    }

}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn run_app() {
    }
}