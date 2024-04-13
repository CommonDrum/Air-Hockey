use bevy::{asset, prelude::*, transform};
use bevy::window::PrimaryWindow;
use rand::Rng;

pub const MAP_SIZE: i32 = 10;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, ((spawn_player, spawn_camera).chain(), (generate_map, render_map).chain()))
        .add_systems(Update, player_movement)
        .run();
}
#[derive(Component)]
struct Player{
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Map{
    map: Vec<Vec<i32>>,
    tile_size: f32,
}


fn generate_map(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>

){
    let window = window_query.get_single().unwrap();

    let tile_size = window.height() / MAP_SIZE as f32;

    let mut map = vec![vec![0; MAP_SIZE as usize]; MAP_SIZE as usize];
    for y in 0..MAP_SIZE{
        for x in 0..MAP_SIZE{
            if x == 0 || x == MAP_SIZE - 1 || y == 0 || y == MAP_SIZE - 1{
                map[y as usize][x as usize] = 1;
            }
        }
    }
    commands.spawn((Map{map, tile_size},));
}

fn render_map(
    mut commands: Commands, 
    map_query: Query<&Map>, 
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
){
    let window = window_query.get_single().unwrap();
    let map = map_query.get_single().unwrap();

    let element_size = window.height() / map.map.len() as f32;
    let screen_offset = Vec2::new(window.width() / 2.0, window.height() / 2.0) - Vec2::new(element_size * map.map[0].len() as f32 / 2.0, element_size * map.map.len() as f32 / 2.0);

    for (y, row) in map.map.iter().enumerate(){
        for (x, tile) in row.iter().enumerate(){
            let x = x as f32 * element_size + screen_offset.x;
            let y = y as f32 * element_size + screen_offset.y;
            
            let texture = match tile{
                0 => continue,
                1 => asset_server.load("tile_grey.png"),
                _ => asset_server.load("pink_body_circle.png"),
            };

            commands.spawn(SpriteBundle{
                transform: Transform::from_xyz(x + 16.0, y + 16.0, 0.0),
                texture,
                ..default()
            });
        }
    }
}


fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    map_query: Query<&Map>,
) {
    let window = window_query.get_single().unwrap();
    let map= map_query.get_single().unwrap();

    let x = rand::thread_rng().gen_range(1..MAP_SIZE - 1);
    let y = rand::thread_rng().gen_range(1..MAP_SIZE - 1);

    let transform = Transform::from_xyz((window.width() / 2.0, window.height() / 2.0, 0.0) + Vec3::new(x as f32 * map.tile_size, y as f32 * map.tile_size, 0.0);



    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("player.png"),
            ..default()
        },
        Player {x,y},
    ));
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
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
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let tile_size =  window.height() / 10.0;

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

        transform.translation += direction * 10.0 * time.delta_seconds();
    }
}
