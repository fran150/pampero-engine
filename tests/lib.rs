use std::{collections::HashMap, cell::RefCell};

use pampero_engine::entities::EntityDrop;
use pampero_engine::entities::Entity;
use pampero_engine::systems::SystemContext;
use pampero_engine::systems::SystemFunction;
use pampero_engine::GameLoopPhase;
use paste::paste;

use pampero_engine::App;
use pampero_engine::components_gen;

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

    let valen = app.entities_mut().spawn_entity();
    let paksox = app.entities_mut().spawn_entity();

    app.components_mut().add_name(&valen, Name("Valen".to_string()));
    app.components_mut().add_name(&paksox, Name("Paksox".to_string()));

    app.components_mut().add_person(&valen, Person());

    app.register_system(GameLoopPhase::Physics, SystemFunction::from(greet_everyone));
    app.register_system(GameLoopPhase::Physics, SystemFunction::from(sit_persons));

    app.run();
}
