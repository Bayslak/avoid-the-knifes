use bevy::prelude::*;

const PLAYER_SPRITE_PATH: &str = "sprites/skeleton.png";
const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player {
    pub speed: f32,
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