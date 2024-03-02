
use pampero_engine::{
    core::GameLoop,
    ecs::{
        Entity,
        SystemContext
    },
    events::{
        Event,
        GameLoopEventPhase,
        SystemEvents
    },
    App,
    generate_components_struct
};

#[derive(Debug)]
pub struct Person();

pub struct Name(String);

pub struct Seated();

pub struct Inside();

generate_components_struct!(
    Components,
    person: Person, 
    name: Name,
    seated: Seated,
    inside: Inside
);

fn sit_persons(context: SystemContext<Components>) {
    let filtered: Vec<&Entity> = context.entities.iter()
        .filter(|e| {
            context.components.get_name(e).is_some() && 
            context.components.get_person(e).is_some() &&
            context.components.get_seated(e).is_none() &&
            context.components.get_inside(e).is_none()
        }).collect();
        
    filtered.iter().for_each(| entity | {
        {        
            let name = context.components.get_name(entity).unwrap();
            let person = context.components.get_person(entity).unwrap();

            println!("[Type: {:?}] Please {}, come in!!!", person, name.0);
        };

        let is_valen = {
            let mut result = false;

            let name = context.components.get_name_mut(entity).unwrap();

            if name.0 == "Valen" {    
                name.0.push_str(" (Seated)");
                result = true;
            }

            result
        };

        if is_valen {
            context.components.add_seated(entity, Seated());
        } else {
            context.components.add_inside(entity, Inside());
        }
    });
}

#[test]
fn run_app() {   
    let default_group = "Default";

    let components = Components::new();
    let mut app = App::new(components);

    let valen = app.ecs.entities.spawn_entity();
    let paksox = app.ecs.entities.spawn_entity();
    let otro = app.ecs.entities.spawn_entity();

    app.ecs.components.add_name(&valen, Name("Valen".to_string()));
    app.ecs.components.add_name(&otro, Name("Otro".to_string()));
    app.ecs.components.add_name(&paksox, Name("Paksox".to_string()));

    app.ecs.components.add_person(&valen, Person());
    app.ecs.components.add_person(&otro, Person());

    app.ecs.systems.register_system(default_group.to_string(), |context| {
        for entity in context.entities.iter() {
            if let Some(name) = context.components.get_name(entity) {
                println!("Hello! Welcome {}", name.0);
            }
        }
    });

    app.ecs.systems.register_system(default_group.to_string(), sit_persons);

    let mut game_loop = GameLoop::new();

    game_loop.handlers.set(GameLoopEventPhase::Physics, |context| {
        context.app.ecs.run_systems("Default".to_string(), context.event);
    });

    game_loop.handlers.set(GameLoopEventPhase::PostLoop, |context| {
        if let Event::SystemEvent(SystemEvents::GameLoopEvent { event_type: _, t, dt: _}) = context.event {
            if *t > 100.0 {
                context.app.stop();
            }
        }
    });

    app.run(&mut game_loop);
}
