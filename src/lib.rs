use core::GameLoop;

use ecs::{
    ECS, 
    EntityDrop
};
use events::Events;

pub mod ecs;
pub mod core;
pub mod events;

mod macros;

pub struct App<T, U> {
    run: bool,
    pub ecs: ECS<T, U>,
    pub events: Events<U>
}

impl<T, U> App<T, U> {
    
    pub fn new(components: T) -> Self
        where T: EntityDrop {
        App {
            run: false,
            ecs: ECS::new(components),
            events: Events::new()
        }
    }

    pub fn is_running(&self) -> bool {
        self.run
    }

    pub fn run(&mut self, game_loop: &mut GameLoop<T, U>) 
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