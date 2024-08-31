use bevy::{input::keyboard::KeyboardInput, prelude::*};

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

// Sprites
const PLAYER_SPRITE_PATH: &str = "sprites/skeleton.png";
const KNIFE_SPRITE_PATH: &str = "sprites/knife.png";

const SPRITE_W: usize = 16;
const SPRITE_H: usize = 16;

// Player
const PLAYER_SPEED: f32 = 500.0;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            resizable: true,
                            focused: true,
                            resolution: (WW, WH).into(),
                            title: "Avoid the Knifes".to_string(),
                            ..default()
                        }),
                        ..default()
                    }),
            )
    .insert_resource(Msaa::Off)
    .add_systems(Startup, (setup_camera, spawn_player))
    .add_systems(Update, player_movement) 
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE_PATH),
        transform: Transform {
            scale: Vec3::splat(4.0),
            ..default()
        },
        ..default()
    }, 
        Player { speed: PLAYER_SPEED })).insert(Name::new("Player"));
}

#[derive(Component)]
struct Player {
    speed: f32,
}

fn player_movement(mut player: Query<(&mut Transform, &Player)>, input: Res<ButtonInput<KeyCode>>, time: Res<Time>,) {

    for (mut transform, player) in &mut player {
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += player.speed * time.delta_seconds();
        }
        
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
    }
}