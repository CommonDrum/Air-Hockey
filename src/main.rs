use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 50.0;
pub const ENEMY_SIZE: f32 = 50.0;
pub const ENEMY_SPEED: f32 = 100.0;
pub const INITIAL_ENEMY_COUNT: u32 = 225;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), spawn_enemy))
        .add_systems(Update, (player_movement, enemy_movement))
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy{
    direction: Vec3,
}


pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();


    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player {},
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..INITIAL_ENEMY_COUNT {

    let x = rand::thread_rng().gen_range(0.0..window.width());
    let y = rand::thread_rng().gen_range(0.0..window.height());

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.0),
            texture: asset_server.load("enemy.png"),
            ..default()
        },
        Enemy {
            direction: Vec3::new(rand::thread_rng().gen_range(-1.0..1.0), rand::thread_rng().gen_range(-1.0..1.0), 0.0).normalize(),
        },
    ));
}
}



pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}


fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();
    
    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        transform.translation += enemy.direction * ENEMY_SPEED * time.delta_seconds();
        if transform.translation.x < 0.0 || transform.translation.x > window.width() || transform.translation.y < 0.0 || transform.translation.y > window.height() {
            if rand::thread_rng().gen_range(0.0..1.0) > 0.5 {
                enemy.direction.x = -enemy.direction.x;
            } else {
                enemy.direction.y = -enemy.direction.y;
            }
        }
    }
}