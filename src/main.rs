use bevy::prelude::*;

mod load_gltf;

mod camera_orbit;
mod movement;
mod player_controller;
// mod mesh_deformation;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);


fn hello_world() {
    println!("hello world!");
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    // update our timer with the time elapsed since the last update
    // if that caused the timer to finish, we say hello to everyone
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}


fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (update_people, greet_people).chain());
    }
}


fn main() {
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugins(HelloPlugin)
    //     .run();

    // App::new()
    // .add_plugins(DefaultPlugins)
    // .add_systems(
    //     Startup,
    //     (
    //         fpvm::spawn_view_model,
    //         fpvm::spawn_world_model,
    //         fpvm::spawn_lights,
    //         fpvm::spawn_text,
    //     ),
    // )
    // .add_systems(Update, (fpvm::move_player, fpvm::move_camera, fpvm::change_fov))
    // .run();

    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_plugins(load_gltf::LoadGltfPlugin)
    //     .run();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player_controller::PlayerControllerPlugin)
        .add_plugins(camera_orbit::CameraOrbitPlugin)
        // .add_plugins(mesh_deformation::MeshDeformationPlugin)
        .run();

}