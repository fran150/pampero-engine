use core::GameLoop;

use ecs::{ECSSystem, EntityDrop};

pub mod ecs;
pub mod core;
pub mod event;

mod macros;

pub struct App {
    run: bool,
}

impl App {
    
    pub fn new() -> Self {
        App {
            run: false,
        }
    }

    pub fn is_running(&self) -> bool {
        self.run
    }

    pub fn run<T>(&mut self, ecs: &mut ECSSystem<T>, game_loop: &mut GameLoop<T>) 
        where T: EntityDrop {
        self.run = true;
        game_loop.run(self, ecs);
    }

}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn run_app() {
    }
}