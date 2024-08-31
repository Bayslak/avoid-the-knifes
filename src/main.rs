mod player;
mod terrain;
mod gravity;
mod player_input;
mod movement;

use bevy::prelude::*;
use gravity::GravityPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use player_input::{InputPlugin, MovementInputEvent};
use terrain::TerrainPlugin;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

// Sprites
const KNIFE_SPRITE_PATH: &str = "sprites/knife.png";

const SPRITE_W: usize = 16;
const SPRITE_H: usize = 16;

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            focused: true,
                            resolution: (WW, WH).into(),
                            title: "Avoid the Knifes".to_string(),
                            ..default()
                        }),
                        ..default()
                    }),
            )
    .add_plugins((InputPlugin, MovementPlugin, TerrainPlugin, PlayerPlugin, GravityPlugin))
    .insert_resource(Msaa::Off)
    .add_event::<MovementInputEvent>()
    .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}