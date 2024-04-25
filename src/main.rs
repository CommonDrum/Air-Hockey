
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

pub const INITIAL_ENEMY_COUNT: u32 = 1;

pub const SHOOT_BASE_STRENGTH: f32 = 1.0;
pub const PLAYER_RANGE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), spawn_enemy, spawn_walls))
        .add_systems(Update, (player_movement, shot_system, lock_in, keep_next_to_player))
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Lock{
    locked: bool,
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
    .insert(ColliderMassProperties::Density(0.01))
    .insert(
        (Enemy{}, Lock{locked: false})
    )
    .insert(ExternalForce {
        force: Vec2::new(0.0, 0.0),
        torque: 0.0,
    })
    .insert(ExternalImpulse {
        impulse: Vec2::new(0.0, 0.0),
        torque_impulse: 0.0,
    });
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

fn shot_system(
    mut enemy_query: Query<(&mut ExternalImpulse, &Transform, &mut Lock)>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    let player_transform = player_query.get_single().unwrap();
    let player_pos = player_transform.translation;
    
    if keyboard_input.just_pressed(KeyCode::Space) {


        for (mut ext_impulse, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            enemy.locked = false;
            let enemy_pos = enemy_transform.translation;
            let direction = player_pos - enemy_pos;
            if direction.length() <= PLAYER_SIZE + ENEMY_SIZE + PLAYER_RANGE {

                let direction = direction.normalize_or_zero(); // Use normalize_or_zero to avoid panics if the direction is a zero vector
                ext_impulse.impulse = Vec2::new(-direction.x, -direction.y) * SHOOT_BASE_STRENGTH;
            }

       }
    }
}


fn lock_in(
    mut enemy_query: Query<(& Transform, &mut Lock), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    let player_transform = player_query.get_single().unwrap();
    let player_pos = player_transform.translation;

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation;
        let direction = player_pos - enemy_pos;
        if direction.length() <= PLAYER_SIZE + ENEMY_SIZE + PLAYER_RANGE && keyboard_input.just_pressed(KeyCode::KeyE) {
            enemy.locked = true;
        }   
    }

}

fn keep_next_to_player(
    mut set: ParamSet<(
        Query<(&mut Lock, &mut Transform), With<Enemy>>,
        Query<&Transform, With<Player>>,
    )>
){
    let player_pos = {
        let player_transform = set.p1().get_single().unwrap().clone(); // Temporarily borrow `set` to get player transform
        player_transform.translation // Copy the position data
    };

    for (lock, mut transform) in set.p0().iter_mut() { // This is now the only active mutable borrow
        if lock.locked {
            transform.translation = player_pos + Vec3::new(PLAYER_SIZE + ENEMY_SIZE, 0.0, 0.0);
        }
    }
}