mod player;
mod terrain;
mod gravity;
mod movement;
mod knife;
mod points;
mod ui;
mod coin;

use bevy::prelude::*;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use coin::coin::{CoinAnimationAssets, CoinPlugin};
use coin::coin_spawner::CoinSpawnerPlugin;
use gravity::gravity::GravityPlugin;
use knife::knife::KnifePlugin;
use knife::knife_spawner::KnifeSpawnerPlugin;
use movement::movement::MovementPlugin;
use player::player_input::InputPlugin;
use player::player::{PlayerAnimationAssets, PlayerPlugin};
use points::points::PointsPlugin;
use terrain::terrain::TerrainPlugin;
use ui::main_menu::MainMenuPlugin;
use ui::ui::UIPlugin;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

const SPRITE_W: usize = 16;
const SPRITE_H: usize = 16;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    AssetLoading,
    Menu,
    Game
}

#[derive(Component)]
pub struct CleanupGameStateExit;

#[derive(Component)]
pub struct CleanupMenuStateExit;

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
    .add_plugins((InputPlugin { state: GameState::Game }, MovementPlugin { state: GameState::Game }, TerrainPlugin { state: GameState::Game }, GravityPlugin { state: GameState::Game }))
    .add_plugins(PointsPlugin)
    .add_plugins((UIPlugin { state: GameState::Game }, MainMenuPlugin { state: GameState::Menu }))
    .add_plugins(PlayerPlugin { state: GameState::Game })
    .add_plugins((CoinPlugin { state: GameState::Game }, CoinSpawnerPlugin { state: GameState::Game }))
    .add_plugins((KnifePlugin { state: GameState::Game }, KnifeSpawnerPlugin { state: GameState::Game }))
    .add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Menu)
        .load_collection::<CoinAnimationAssets>()
        .load_collection::<PlayerAnimationAssets>()
    )
    .add_systems(OnExit(GameState::Menu), cleanup_system::<CleanupMenuStateExit>)
    .add_systems(OnExit(GameState::Game), cleanup_system::<CleanupGameStateExit>)
    .insert_resource(Msaa::Off)
    .init_state::<GameState>()
    .add_systems(Startup, setup_camera)
    .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn cleanup_system<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for entity in q.iter() {
        commands.entity(entity).despawn_recursive();
    }
}