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
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), spawn_enemy))
        .add_systems(Update, (player_movement, enemy_movement, confine_player, confine_enemies, collision_detection, shoot ))
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

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                translation: Vec3::new(x, y, 0.0),
                ..default()
            },
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


fn enemy_movement(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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

            println!("Enemy out of bounds");

            commands.spawn((
                AudioBundle {
                    source: asset_server.load("pluck.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                },
            ));
        }

           
    }
}

fn confine_player(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    for mut transform in player_query.iter_mut() {
        if transform.translation.x + PLAYER_SIZE / 2.0 > window.width() {
            transform.translation.x = window.width() - PLAYER_SIZE / 2.0;
        }
        if transform.translation.x - PLAYER_SIZE / 2.0 < 0.0 {
            transform.translation.x = PLAYER_SIZE / 2.0;
        }
        if transform.translation.y + PLAYER_SIZE / 2.0 > window.height() {
            transform.translation.y = window.height() - PLAYER_SIZE / 2.0;
        }
        if transform.translation.y - PLAYER_SIZE / 2.0 < 0.0 {
            transform.translation.y = PLAYER_SIZE / 2.0;
        }
    }
}

fn confine_enemies(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let mut colided = false;

    for (mut transform, mut enemy) in enemy_query.iter_mut() {
        if transform.translation.x + ENEMY_SIZE / 2.0 > window.width() {
            transform.translation.x = window.width() - ENEMY_SIZE / 2.0;
            enemy.direction.x = -enemy.direction.x;
            colided = true;
        }
        if transform.translation.x - ENEMY_SIZE / 2.0 < 0.0 {
            transform.translation.x = ENEMY_SIZE / 2.0;
            enemy.direction.x = -enemy.direction.x;
            colided = true;
        }
        if transform.translation.y + ENEMY_SIZE / 2.0 > window.height() {
            transform.translation.y = window.height() - ENEMY_SIZE / 2.0;
            enemy.direction.y = -enemy.direction.y;
            colided = true;
        }
        if transform.translation.y - ENEMY_SIZE / 2.0 < 0.0 {
            transform.translation.y = ENEMY_SIZE / 2.0;
            enemy.direction.y = -enemy.direction.y;
            colided = true;
        }

       // if colided {
       // commands.spawn((
       //     AudioBundle {
       //         source: asset_server.load("pluck.ogg"),
       //         settings: PlaybackSettings::DESPAWN,
       //     },
       // ));
       //}
    }
}

#[derive(Component)]
struct AnimateScale;

fn collision_detection(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for player_transform in player_query.iter() {
        for enemy_transform in enemy_query.iter() {
            if player_transform.translation.distance(enemy_transform.translation) < PLAYER_SIZE / 2.0 + ENEMY_SIZE / 2.0 {

                let font = asset_server.load("milker.otf");


                let text_style = TextStyle {
                    font: font.clone(),
                    font_size: 60.0,
                    color: Color::WHITE,
                };
                let text_justification = JustifyText::Right;
            
                commands.spawn((
                    Text2dBundle {
                        text: Text::from_section("You Lost!", text_style).with_justify(text_justification),
                        ..default()
                    },
                    AnimateScale {},
                ));            
            }
        }
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
