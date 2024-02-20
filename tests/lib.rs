
use pampero_engine::core::GameLoop;
use pampero_engine::ecs::ECSSystem;
use pampero_engine::ecs::SystemContext;
use pampero_engine::ecs::SystemFunction;
use pampero_engine::core::GameLoopPhase;

use pampero_engine::App;
use pampero_engine::components_gen;

#[derive(Debug)]
pub struct Person();

pub struct Name(String);

components_gen!(person: Person, name: Name);

fn greet_everyone(context: SystemContext<Components>) {
    for entity in context.entities.iter() {
        if let Some(name) = context.components.get_name(entity) {
            println!("Hello! Welcome {}", name.borrow().0);
        }
    }
}


fn sit_persons(context: SystemContext<Components>) {
    context.entities.iter()
        .filter(|e| {
            context.components.get_name(e).is_some() && 
            context.components.get_person(e).is_some()
        })
        .for_each(|entity| {
            let name = context.components.get_name(entity).unwrap();
            let person = context.components.get_person(entity).unwrap();
    
            let mut name = name.borrow_mut();
    
            name.0.push_str(" (Seated)");
        
            println!("[Type: {:?}] Please {}, sit here!!!", person.borrow_mut(), name.0);
        });
}


#[test]
fn run_app() {    
    let mut app = App::new();
    let components = Components::new();
    let mut ecs = ECSSystem::new(components);

    let valen = ecs.entities_mut().spawn_entity();
    let paksox = ecs.entities_mut().spawn_entity();

    ecs.components_mut().add_name(&valen, Name("Valen".to_string()));
    ecs.components_mut().add_name(&paksox, Name("Paksox".to_string()));

    ecs.components_mut().add_person(&valen, Person());

    ecs.systems_mut().register_system(GameLoopPhase::Physics, SystemFunction::from(greet_everyone));
    ecs.systems_mut().register_system(GameLoopPhase::Physics, SystemFunction::from(sit_persons));

    let mut game_loop = GameLoop::new();

    app.run(&mut ecs, &mut game_loop);
}
