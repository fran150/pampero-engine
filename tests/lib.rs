
use pampero_engine::{
    game_loop::{
        GameLoop,
        GameLoopPhase,
    }, ecs::{
        Entity,
        SystemContext
    }, events::{
        Event, SystemEvents
    }, generate_components_struct, App
};

pub enum UserEvents {
    GoAway {
        name: Name,    
    },
}

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

fn sit_persons(context: SystemContext<Components, UserEvents>) {
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
    const DEFAULT_GROUP: &str = "Default";
    const CLOSING_GROUP: &str = "Closing";

    let components = Components::new();
    let mut app = App::new(components);

    let valen = app.ecs.spawn_entity();
    let paksox = app.ecs.spawn_entity();
    let otro = app.ecs.spawn_entity();

    app.ecs.components.add_name(&valen, Name("Valen".to_string()));
    app.ecs.components.add_name(&otro, Name("Otro".to_string()));
    app.ecs.components.add_name(&paksox, Name("Paksox".to_string()));

    app.ecs.components.add_person(&valen, Person());
    app.ecs.components.add_person(&otro, Person());

    app.ecs.register_system(DEFAULT_GROUP.to_string(), |context| {
        for entity in context.entities.iter() {
            if let Some(name) = context.components.get_name(entity) {
                println!("Hello! Welcome {}", name.0);
            }
        }
    });

    app.ecs.register_system(CLOSING_GROUP.to_string(), |context| {
        context.entities.iter().filter(|entity| {
            context.components.get_name(entity).is_some()
        }).for_each(|entity| {
            if let Event::UserEvent(UserEvents::GoAway { name }) = context.event {
                let current = context.components.get_name(entity).unwrap();
                if current.0 == name.0 {
                    println!("Goodbye! {}", name.0);
                }
            }
        });
    });

    app.ecs.register_system(DEFAULT_GROUP.to_string(), sit_persons);

    let mut game_loop = GameLoop::new();

    game_loop.handlers.set(GameLoopPhase::Physics, |context| {
        context.app.ecs.run_systems(DEFAULT_GROUP.to_string(), context.event);
    });

    game_loop.handlers.set(GameLoopPhase::PostLoop, |context| {
        if let Some(event) = context.app.events.pop() {
            context.app.ecs.run_systems(CLOSING_GROUP.to_string(), &event);
        }
    });

    game_loop.handlers.set(GameLoopPhase::PreLoop, |context| {
        if let Event::SystemEvent(SystemEvents::GameLoopEvent { phase: _, t, dt: _}) = context.event {
            if *t > 100.0 {
                context.app.events.dispatch(Event::user_event(
                    UserEvents::GoAway { name: Name("Paksox".to_string())}
                ));
                context.app.stop();
            }
        }
    });

    app.run(&mut game_loop);
}
