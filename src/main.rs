mod player;
mod terrain;
mod gravity;
mod movement;
mod knife;
mod points;
mod ui;
mod coin;

use bevy::prelude::*;
use coin::coin::CoinPlugin;
use coin::coin_spawner::CoinSpawnerPlugin;
use gravity::gravity::GravityPlugin;
use knife::knife::KnifePlugin;
use knife::knife_spawner::KnifeSpawnerPlugin;
use movement::movement::MovementPlugin;
use player::player_input::InputPlugin;
use player::player::PlayerPlugin;
use points::points::PointsPlugin;
use terrain::terrain::TerrainPlugin;
use ui::ui::UIPlugin;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;


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
    .add_plugins((InputPlugin, MovementPlugin, TerrainPlugin, GravityPlugin))
    .add_plugins(PointsPlugin)
    .add_plugins(UIPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins((CoinPlugin, CoinSpawnerPlugin))
    .add_plugins((KnifePlugin, KnifeSpawnerPlugin))
    .insert_resource(Msaa::Off)
    .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}