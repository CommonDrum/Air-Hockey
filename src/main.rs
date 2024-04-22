use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const ENEMY_SCALE: f32 = 1.0;
pub const PLAYER_SCALE: f32 = 1.0;
pub const BULLET_SCALE: f32 = 1.0;

pub const PLAYER_SIZE: f32 = 50.0 * PLAYER_SCALE;
pub const BULLET_SIZE: f32 = 10.0 * BULLET_SCALE;
pub const ENEMY_SIZE: f32 = 50.0 * ENEMY_SCALE;

pub const PLAYER_SPEED: f32 = 500.0;
pub const ENEMY_SPEED: f32 = 100.0;

pub const INITIAL_ENEMY_COUNT: u32 = 4;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), spawn_enemy, spawn_walls))
        .add_systems(Update, (player_movement, shoot))
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

    commands.spawn(RigidBody::KinematicPositionBased)
    .insert(Collider::ball(PLAYER_SIZE / 2.0))
    .insert(SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        texture: asset_server.load("player.png"),
        ..default()
    })
    .insert(Player)
    .insert(TransformBundle::from(Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)))
    .insert(KinematicCharacterController::default());
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

    commands.spawn(RigidBody::Dynamic)
    .insert(Collider::ball(ENEMY_SIZE / 2.0))
    .insert(SpriteBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        texture: asset_server.load("enemy.png"),
        ..default()
    })
    .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
    .insert(GravityScale(0.0))
    .insert(Restitution::coefficient(0.9))
    .insert(Velocity {
        linvel: Vec2::new(x, y).normalize() * ENEMY_SPEED,
        angvel: 0.4,})
    .insert(ColliderMassProperties::Density(0.01))
    .insert(
        Enemy {
            direction: Vec3::new(rand::thread_rng().gen_range(-1.0..1.0), rand::thread_rng().gen_range(-1.0..1.0), 0.0).normalize(),
        },
    );
}
}

fn spawn_walls(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn(RigidBody::Fixed)
    .insert(Collider::cuboid(window.width(), 10.0))
    .insert(TransformBundle::from(Transform::from_xyz(window.width() / 2.0, 0.0, 0.0)))
    .insert(Restitution::coefficient(0.7));

    commands.spawn(RigidBody::Fixed)
    .insert(Collider::cuboid(window.width(), 10.0))
    .insert(TransformBundle::from(Transform::from_xyz(window.width() / 2.0, window.height(), 0.0)))
    .insert(Restitution::coefficient(0.7));

    commands.spawn(RigidBody::Fixed)
    .insert(Collider::cuboid(10.0, window.height()))
    .insert(TransformBundle::from(Transform::from_xyz(0.0, window.height() / 2.0, 0.0)))
    .insert(Restitution::coefficient(0.7));

    commands.spawn(RigidBody::Fixed)
    .insert(Collider::cuboid(10.0, window.height()))
    .insert(TransformBundle::from(Transform::from_xyz(window.width(), window.height() / 2.0, 0.0)))
    .insert(Restitution::coefficient(0.7));


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
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,
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

        let movement = Some(Vec2::new(direction.x, direction.y) * PLAYER_SPEED * time.delta_seconds());

        transform.translation = movement;
    }
}

fn shoot(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {

        let player_transform = player_query.get_single().unwrap();

        commands.spawn(SpriteBundle {
                    transform: Transform {
                        translation: player_transform.translation,
                        ..default()
                    },
                    texture: asset_server.load("bullet.png"),
                    ..default()
                });
        }
}


fn attraction(
    mut commands: Commands,
    mut player_query: Query<&mut KinematicCharacterController, With<Player>>,

) {
}
