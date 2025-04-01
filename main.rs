use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Gravity(f32);

const MOVEMENT_SPEED: f32 = 200.0;
const GRAVITY: f32 = -50.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.576, 0.671, 0.761)))
        .add_plugins(DefaultPlugins)
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
    mut sprite_position: Query<&mut Transform, With<Player>>,
){
    for mut transform in &mut sprite_position {
        let mut direction = Vec2::ZERO;
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if direction.length() > 0.0 {
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
        gravity.0 -= 2.;
        transform.translation += Vec3::new(0., 1., 0.) * gravity.0 * time.delta_secs();
    }
}

