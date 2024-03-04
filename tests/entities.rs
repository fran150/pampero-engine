
use pampero_engine::{
    App,
    generate_components_struct
};

pub struct Name(String);
pub struct Person();

generate_components_struct!(
    Components, 
    name: Name,
    person: Person
);

#[test]
fn entities_can_be_created_and_removed() {   
    let components = Components::new();
    let mut app = App::new(components);

    let e1 = app.ecs.spawn_entity();
    let e2 = app.ecs.spawn_entity();
    let e3 = app.ecs.spawn_entity();

    assert_eq!(true, app.ecs.entities.is_registered(&e1));
    assert_eq!(true, app.ecs.entities.is_registered(&e2));
    assert_eq!(true, app.ecs.entities.is_registered(&e3));

    app.ecs.remove_entity(&e1);

    assert_eq!(false, app.ecs.entities.is_registered(&e1));

    let remaining_entities = vec![e2, e3];

    let all_exists = app.ecs.entities.iter()
        .all(|value| { remaining_entities.contains(value) });

    assert_eq!(true, all_exists);
}

#[test]
fn entities_iterator_to_select_based_on_components() {
    let components = Components::new();
    let mut app = App::new(components);

    let person = app.ecs.spawn_entity();
    let cat = app.ecs.spawn_entity();
    let _rock = app.ecs.spawn_entity();

    app.ecs.components.add_name(&person, Name(String::from("Valen")));
    app.ecs.components.add_name(&cat, Name(String::from("Benito")));
    app.ecs.components.add_person(&person, Person());

    // Returns only entities that have name and person components
    let only_valen = app.ecs.entities.iter()
        .filter(|entity| {
            app.ecs.components.get_name(entity).is_some() &&
            app.ecs.components.get_person(entity).is_some()
        }).all(|entity| {
            entity == &person
        });

    assert_eq!(true, only_valen);
}
