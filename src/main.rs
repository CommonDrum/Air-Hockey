
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const BALL_SCALE: f32 = 1.0;
pub const PLAYER_SCALE: f32 = 1.0;
pub const BULLET_SCALE: f32 = 1.0;

pub const PLAYER_SIZE: f32 = 60.0 * PLAYER_SCALE;
pub const BALL_SIZE: f32 = 70.0 * BALL_SCALE;

pub const PLAYER_SPEED: f32 = 500.0;
pub const BALL_SPEED: f32 = 100.0;

pub const INITIAL_BALL_COUNT: u32 = 1;

pub const SHOOT_BASE_STRENGTH: f32 = 1.0;
pub const PLAYER_RANGE: f32 = 10.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), spawn_ball, spawn_walls))
        .add_systems(Update, (player_movement, shot_system, lock_in, keep_next_to_player, score))
        .run();
}
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Lock{
    locked: bool,
}

#[derive(Component)]
struct ScoreText{
    score: u32,
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

fn spawn_ball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..INITIAL_BALL_COUNT {

    let x = rand::thread_rng().gen_range(0.0..window.width());
    let y = rand::thread_rng().gen_range(0.0..window.height());

    commands.spawn(RigidBody::Dynamic)
    .insert(Collider::ball(BALL_SIZE / 2.0))
    .insert(SpriteBundle {
        transform: Transform::from_xyz(x, y, 0.0),
        texture: asset_server.load("ball.png"),
        ..default()
    })
    .insert(TransformBundle::from(Transform::from_xyz(x, y, 0.0)))
    .insert(GravityScale(0.0))
    .insert(Restitution::coefficient(0.9))
    .insert(ColliderMassProperties::Density(0.01))
    .insert(
        (Ball{}, Lock{locked: false})
    )
    .insert(ExternalForce {
        force: Vec2::new(0.0, 0.0),
        torque: 0.0,
    })
    .insert(ExternalImpulse {
        impulse: Vec2::new(0.0, 0.0),
        torque_impulse: 0.0,
    })
    .insert(Velocity::default());
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

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, asset_server: Res<AssetServer>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });

    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "SCORE: ",
                TextStyle {
                    font: asset_server.load("Milker.otf"),
                    font_size: 60.0,
                    ..default()
                },
            ),
            TextSection::from_style(if cfg!(feature = "default_font") {
                TextStyle {
                    font_size: 60.0,
                    color: Color::GOLD,
                    ..default()
                }
            } else {
                TextStyle {
                    font: asset_server.load("Milker.otf"),
                    font_size: 60.0,
                    color: Color::GOLD,
                }
            }),
        ]),
        ScoreText{score: 0},
    ));

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
    mut ball_query: Query<(&mut ExternalImpulse, &Transform, &mut Lock)>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    let player_transform = player_query.get_single().unwrap();
    let player_pos = player_transform.translation;
    
    if keyboard_input.just_pressed(KeyCode::Space) {


        for (mut ext_impulse, ball_transform, mut ball) in ball_query.iter_mut() {
            ball.locked = false;
            let ball_pos = ball_transform.translation;
            let direction = player_pos - ball_pos;
            if direction.length() <= PLAYER_SIZE + BALL_SIZE + PLAYER_RANGE {

                let direction = direction.normalize_or_zero(); 
                ext_impulse.impulse = Vec2::new(-direction.x, -direction.y) * SHOOT_BASE_STRENGTH;
            }

       }
    }
}


fn lock_in(
    mut ball_query: Query<(& Transform, &mut Lock, &mut Velocity), With<Ball>>,
    player_query: Query<&Transform, With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>
){
    let player_transform = player_query.get_single().unwrap();
    let player_pos = player_transform.translation;

    for (ball_transform, mut ball, mut vel) in ball_query.iter_mut() {
        let ball_pos = ball_transform.translation;
        let direction = player_pos - ball_pos;
        if direction.length() <= PLAYER_SIZE + BALL_SIZE + PLAYER_RANGE && keyboard_input.just_pressed(KeyCode::KeyE) {
            vel.linvel = Vec2::ZERO;
            vel.angvel = 0.0;
            ball.locked = true;
            
        }   
    }

}

fn keep_next_to_player(
    mut set: ParamSet<(
        Query<(&mut Lock, &mut Transform), With<Ball>>,
        Query<&Transform, With<Player>>,
    )>
){
    let player_pos = {
        let player_transform = set.p1().get_single().unwrap().clone(); 
        player_transform.translation 
    };

    for (lock, mut transform) in set.p0().iter_mut() { 
        if lock.locked {
            transform.translation = player_pos + Vec3::new(PLAYER_SIZE + BALL_SIZE, 0.0, 0.0);
        }
    }
}



fn score(
    ball_query: Query<&Transform, With<Ball>>,
    mut score_query: Query<(&mut ScoreText, &mut Text)>,
    
){

    let mut score = 0;
    for ball_transform in ball_query.iter() {
        let ball_pos = ball_transform.translation;
        if ball_pos.x < 0.0 || ball_pos.x > 800.0 || ball_pos.y < 0.0 || ball_pos.y > 600.0 {
            score += 1;
        }
    }

    for (mut score_text, mut text) in score_query.iter_mut() {
        score_text.score += score;
        text.sections[1].value = score_text.score.to_string();
    }
    
}