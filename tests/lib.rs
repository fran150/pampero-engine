use std::{collections::HashMap, cell::RefCell};

use paste::paste;

use pampero_engine::Entity;
use pampero_engine::App;
use pampero_engine::components_gen;
use pampero_engine::EntityDrop;

#[derive(Debug)]
pub struct Person();

pub struct Name(String);

components_gen!(person: Person, name: Name);

fn greet_everyone(entity: Entity, components: &mut Components) -> Option<()> {
    let name = components.get_name(&entity)?;

    println!("Hello! Welcome {}", name.borrow().0);
    Some(())
}

fn sit_persons(entity: Entity, components: &mut Components) -> Option<()> {
    let name = components.get_name(&entity)?;
    let person = components.get_person(&entity)?;

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

    app.register_system(greet_everyone);
    app.register_system(sit_persons);

    app.run();
}
