use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use std::sync::Once;

macro_rules! using_stub_function {
    ($func_name:expr) => {
        let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.6fZ").to_string();
        println!(
            "\x1b[38;2;145;147;150m{}  \x1b[93mWARN\x1b[0m \x1b[38;2;145;147;150m{}: \x1b[0mUsing unimplemented '\x1b[96m{}()\x1b[0m' function as stub",
            timestamp, module_path!(), $func_name
        );
    };
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gravity(f32);

const MOVEMENT_SPEED: f32 = 120.;
const GRAVITY: f32 = -50.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.576, 0.671, 0.761)))
        .add_plugins(DefaultPlugins)
        .add_plugins(DebugPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, (handle_input, handle_gravity))
        .run();
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    commands.spawn(Camera2d);
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50., 50.))),
        MeshMaterial2d(materials.add(Color::hsl(208., 0.75, 0.61))),
        Transform::from_xyz(0., -200., 0.),
        Player,
        Gravity(GRAVITY),
    ));
}

fn handle_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut Gravity), With<Player>>,
){
    for (mut transform, mut gravity) in &mut sprite_position {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }
        if keyboard_input.just_pressed(KeyCode::KeyW) {
            if helper_is_on_ground(){
                gravity.0 = 98.;
            }
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }
        if direction.length() > 0. {
            direction = direction.normalize();
        }
        transform.translation += Vec3::new(direction.x, direction.y, 0.0) * MOVEMENT_SPEED * time.delta_secs();
    }
}

fn handle_gravity( // don't mean too much to me!
    time: Res<Time>,
    mut sprite_position: Query<(&mut Transform, &mut Gravity), With<Player>>,
){
    for (mut transform, mut gravity) in &mut sprite_position {
        if gravity.0 > -167. {gravity.0 -= 2.;}
        else {gravity.0 = -167.;}
        transform.translation += Vec3::new(0., 1., 0.) * gravity.0 * time.delta_secs();
    }
}

// debug functions

pub struct DebugPlugins;

impl Plugin for DebugPlugins {
    fn build(&self, app: &mut App) {
           app.add_systems(Update, debug_wraparound);
    }
}


fn debug_wraparound(
    mut sprite_position: Query<&mut Transform, With<Player>>,
    windows: Query<&Window, With<PrimaryWindow>>,
){
    if let Some((width, height)) = helper_get_window_size(&windows) {
        for mut transform in &mut sprite_position {
            if transform.translation.y > height / 2.0 {
                transform.translation.y = -height / 2.0;
            } else if transform.translation.y < -height / 2.0 {
                transform.translation.y = height / 2.0;
            }

            if transform.translation.x > width / 2.0 {
                transform.translation.x = -width / 2.0;
            } else if transform.translation.x < -width / 2.0 {
                transform.translation.x = width / 2.0;
            }
        }
    }
}

// helper functions

fn helper_get_window_size(windows: &Query<&Window, With<PrimaryWindow>>) -> Option<(f32, f32)> {
    if let Ok(window) = windows.get_single() {
        Some((window.width(), window.height()))
    } else {
        None
    }
}

static ONCE: Once = Once::new();

fn helper_is_on_ground() -> bool {
    ONCE.call_once(|| {
        using_stub_function!("helper_is_on_ground");
    });
    true
}
