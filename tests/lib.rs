use std::{collections::HashMap, cell::RefCell};

use pampero_engine::systems::SystemContext;
use pampero_engine::systems::SystemFunction;
use pampero_engine::GameLoopPhase;
use paste::paste;

use pampero_engine::Entity;
use pampero_engine::App;
use pampero_engine::components_gen;
use pampero_engine::EntityDrop;

#[derive(Debug)]
pub struct Person();

pub struct Name(String);

components_gen!(person: Person, name: Name);

fn greet_everyone(context: SystemContext<Components>) -> Option<()> {
    let name = context.components.get_name(&context.entity)?;

    println!("Hello! Welcome {}", name.borrow().0);
    Some(())
}

fn sit_persons(context: SystemContext<Components>) -> Option<()> {
    let name = context.components.get_name(&context.entity)?;
    let person = context.components.get_person(&context.entity)?;

    let mut name = name.borrow_mut();

    name.0.push_str(" (Seated)");
    
    println!("[Type: {:?}] Please {}, sit here!!!", person.borrow_mut(), name.0);
    Some(())
}


#[test]
fn run_app() {    
    let mut app = App::new(Components::new());

    let valen = app.spawn_entity();
    let paksox = app.spawn_entity();

    app.components().add_name(&valen, Name("Valen".to_string()));
    app.components().add_name(&paksox, Name("Paksox".to_string()));

    app.components().add_person(&valen, Person());

    app.register_system(GameLoopPhase::Physics, SystemFunction::from(greet_everyone));
    app.register_system(GameLoopPhase::Physics, SystemFunction::from(sit_persons));

    app.run();
}
