use bevy::prelude::*;

mod systems;

pub fn run() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(systems::setup::meshes::egg::setup.system())
        .run();
}

struct Person;

struct Name(String);

struct GreetTimer(Timer);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Elaina Proctor".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Renzo Humer".to_string()));

    commands
        .spawn()
        .insert(Person)
        .insert(Name("Zayna Nieve".to_string()));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>, 
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0);
        }    
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_people.system())
            .add_system(greet_people.system());
    }
}