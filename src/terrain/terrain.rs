use bevy::prelude::*;

use crate::{CleanupGameStateExit, GameState};

pub struct TerrainPlugin<GameState: States> {
    pub state: GameState
}

const TERRAIN_SPRITE_PATH: &str = "sprites/terrain.png";

impl Plugin for TerrainPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_terrain.run_if(in_state(self.state.clone())));
    }
}

#[derive(Component)]
pub struct Terrain;

fn spawn_terrain(mut commands: Commands, asset_server: Res<AssetServer>, window_query: Query<&Window>) {

    let window = window_query.get_single().unwrap();

    commands.spawn(( 
            SpriteBundle {
                texture: asset_server.load(TERRAIN_SPRITE_PATH),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(16.0, 16.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(-(window.width() - 16.0) / 2.0, -(window.height() - 16.0) / 2.0, 0.0),
                    scale: Vec3::new(window.width(), 4.0, 0.0),
                    ..default()
                },
                ..default()
            }, Terrain)
        ).insert((Name::new("Terrain"), CleanupGameStateExit));
}