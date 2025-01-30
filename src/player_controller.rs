//! Demonstrates handling a key press/release.

use bevy::{
    color::palettes::tailwind, input::mouse::AccumulatedMouseMotion, pbr::NotShadowCaster,
    prelude::*, render::view::RenderLayers,
};

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, resolve_keyboard_inputs);
        app.add_systems(Update, apply_velocity);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, resolve_keyboard_inputs)
        .run();
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Component, Deref)]
struct Velocity(Vec3);
impl Default for Velocity {
    fn default() -> Self {
        Self(
            Vec3::new(0.0, 0.0, 0.0),
        )
    }
}

/// A vector representing the player's input, accumulated over all frames that ran
/// since the last time the physics simulation was advanced.
#[derive(Debug, Component, Clone, Copy, PartialEq, Default, Deref, DerefMut)]
struct AccumulatedInput(Vec3);
// impl Default for AccumulatedInput {
//     fn default() -> Self {
//         Self(
//             Vec2::new(0.0, 0.0),
//         )
//     }
// }

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,) {
    let body = meshes.add(Cuboid::new(3.0, 3.0, 3.0));
    let body_material = materials.add(Color::from(tailwind::TEAL_200));
    commands.spawn((
        Mesh3d(body),
        MeshMaterial3d(body_material),
        Transform::from_xyz(0.2, -0.1, -0.25),
        Player,
        Velocity::default(),
        AccumulatedInput::default(),
        // Ensure the arm is only rendered by the view model camera.
        // RenderLayers::layer(VIEW_MODEL_RENDER_LAYER),
        // The arm is free-floating, so shadows would look weird.
        NotShadowCaster,
    ));
}

/// This system prints 'A' key state
fn resolve_keyboard_inputs(keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query : Query<(&mut AccumulatedInput, &mut Velocity, &Player)>,
    mut camera: Single<&mut Transform, With<Camera>>,) {
    const SPEED: f32 = 3.0;//player max speed
    //Forward
    for (mut movement_input, mut velocity, player) in query.iter_mut() 
    {
        movement_input.0 = Vec3::new(0.0, 0.0, 0.0);
        if keyboard_input.pressed(KeyCode::KeyW) {
            info!("'W' currently pressed");
            movement_input.z -= 1.0;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            info!("'W' just pressed");
        }
        if keyboard_input.just_released(KeyCode::KeyW) {
            info!("'W' just released");
        }
        //Back
        if keyboard_input.pressed(KeyCode::KeyS) {
            info!("'S' currently pressed");
            movement_input.z += 1.0;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyS) {
            info!("'S' just pressed");
        }
        if keyboard_input.just_released(KeyCode::KeyS) {
            info!("'S' just released");
        }
        //Left
        if keyboard_input.pressed(KeyCode::KeyA) {
            info!("'A' currently pressed");
            movement_input.x -= 1.0;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            info!("'A' just pressed");
        }
        if keyboard_input.just_released(KeyCode::KeyA) {
            info!("'A' just released");
        }
        //Right
        if keyboard_input.pressed(KeyCode::KeyD) {
            info!("'D' currently pressed");
            movement_input.x += 1.0;
        }
    
        if keyboard_input.just_pressed(KeyCode::KeyD) {
            info!("'D' just pressed");
        }
        if keyboard_input.just_released(KeyCode::KeyD) {
            info!("'D' just released");
        }
        let mut rotated_movement_input = camera.rotation*movement_input.0;
        rotated_movement_input = Vec3::new(rotated_movement_input.x, 0.0, rotated_movement_input.z);
        velocity.0 = rotated_movement_input.normalize() * SPEED;
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity, &Player)>, 
    time: Res<Time>)
{
    let lambda = 0.5;//decay factor for velocity
    let delta_time = time.delta_secs();
    let base_lookforward = Vec2::new(0.0, 1.0);
    for (mut transform, mut velocity, player) in query.iter_mut() {
        
        transform.scale = Vec3::new(0.5, 0.5, 1.0);//lazy, should do on setup only
        if(velocity.0.length() > 0.0)
        {
            info!("Velocity: {}", velocity.0);
            transform.translation += velocity.0*delta_time;
            let normalized_velocity_for_rotation = Vec2::new(-velocity.x, velocity.z).normalize();
            let forward_dot_player = base_lookforward.dot(normalized_velocity_for_rotation);
            let theta = base_lookforward.angle_between(normalized_velocity_for_rotation);
            
            let target_rotation = Quat::from_rotation_y(theta);
            let lerp_factor = if transform.rotation.angle_between(target_rotation).abs() > 0.05 {0.05} else {0.5};
            transform.rotation = transform.rotation.lerp(target_rotation, lerp_factor);
            info!("theta.to_degrees: {}", theta.to_degrees());
        }
        velocity.0 = velocity.0*lambda;
    }
}