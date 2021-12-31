use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(setup.system())
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

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let egg_handle: Handle<Mesh> = asset_server.load("egg/scene.gltf#Mesh0/Primitive0");

    let egg_texture_handle: Handle<Texture> = asset_server
        .load("egg/textures/DefaultMaterial_baseColor.jpeg");
    
    let egg_metallic_roughness_texture_handle: Handle<Texture> = asset_server.load("egg/textures/DefaultMaterial_metallicRoughness.png");

    let egg_normal_map_handle: Handle<Texture> = asset_server.load("egg/textures/DefaultMaterial_normal.jpeg");

    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(egg_texture_handle.clone()),
        metallic_roughness_texture: Some(egg_metallic_roughness_texture_handle.clone()),
        normal_map: Some(egg_normal_map_handle.clone()),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: egg_handle,
        material: material_handle,
        transform: Transform::from_xyz(-0.0, 1.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(0.0, 4.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}