mod player;
mod terrain;
mod gravity;
mod movement;
mod knife;
mod points;
mod ui;
mod coin;

use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::loading_state::config::ConfigureLoadingState;
use bevy_asset_loader::loading_state::{LoadingState, LoadingStateAppExt};
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioPlugin;
use bevy_kira_audio::AudioSource as KiraAudioSource;
use coin::coin::{CoinAnimationAssets, CoinPlugin};
use coin::coin_spawner::CoinSpawnerPlugin;
use gravity::gravity::GravityPlugin;
use knife::knife::{KnifeAudios, KnifePlugin};
use knife::knife_spawner::KnifeSpawnerPlugin;
use movement::movement::MovementPlugin;
use player::player_input::InputPlugin;
use player::player::{PlayerAnimationAssets, PlayerAudioSources, PlayerPlugin};
use points::points::{Points, PointsPlugin};
use terrain::terrain::TerrainPlugin;
use ui::main_menu::MainMenuPlugin;
use ui::ui::UIPlugin;

// Window
const WW: f32 = 1200.0;
const WH: f32 = 700.0;

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

#[derive(Resource)]
pub struct Level {
    pub value: i32
}

const LEVEL_UP_TIMER: f32 = 5.0;
#[derive(Resource)]
pub struct LevelIncreaseTimer(Timer);

impl Default for LevelIncreaseTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(LEVEL_UP_TIMER, TimerMode::Repeating))
    }
}

#[derive(Event)]
pub struct LevelUpEvent;

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
    .add_plugins(AudioPlugin)
    .add_plugins((InputPlugin { state: GameState::Game }, MovementPlugin { state: GameState::Game }, TerrainPlugin { state: GameState::Game }, GravityPlugin { state: GameState::Game }))
    .add_plugins(PointsPlugin)
    .add_plugins((UIPlugin { state: GameState::Game }, MainMenuPlugin { state: GameState::Menu }))
    .add_plugins(PlayerPlugin { state: GameState::Game })
    .add_plugins((CoinPlugin { state: GameState::Game }, CoinSpawnerPlugin { state: GameState::Game }))
    .add_plugins((KnifePlugin { state: GameState::Game }, KnifeSpawnerPlugin { state: GameState::Game }))
    .add_audio_channel::<BackgroundChannel>()
    .add_loading_state(
        LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Menu)
        .load_collection::<BackgroundAudios>()
        .load_collection::<CoinAnimationAssets>()
        .load_collection::<PlayerAnimationAssets>().load_collection::<PlayerAudioSources>()
        .load_collection::<KnifeAudios>()
    )
    .add_systems(OnExit(GameState::Menu), cleanup_system::<CleanupMenuStateExit>)
    .add_systems(OnExit(GameState::Game), (cleanup_system::<CleanupGameStateExit>, reset))
    .add_systems(Update, (level_timer_update, play_background_music).run_if(in_state(GameState::Game)))
    .add_event::<LevelUpEvent>()
    .insert_resource(Msaa::Off)
    .insert_resource(Level { value: 0 })
    .init_resource::<LevelIncreaseTimer>()
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

fn level_timer_update(time: Res<Time>, mut leveled_up: EventWriter<LevelUpEvent>, mut level_up_timer: ResMut<LevelIncreaseTimer>, mut level: ResMut<Level>) {
    level_up_timer.0.tick(time.delta());
    
    if level_up_timer.0.just_finished() {
        leveled_up.send(LevelUpEvent);
        level.value += 1;
    }
}

fn reset(mut level: ResMut<Level>, mut points: ResMut<Points>, mut level_timer: ResMut<LevelIncreaseTimer>) {
    level.value = 0;
    points.value = 0;
    level_timer.0 = Timer::from_seconds(LEVEL_UP_TIMER, TimerMode::Repeating);
}

#[derive(Resource)]
pub struct BackgroundChannel;

#[derive(AssetCollection, Resource)]
pub struct BackgroundAudios {
    #[asset(path = "audio/background_music.mp3")]
    background: Handle<KiraAudioSource>
}

fn play_background_music(background_audios: Res<BackgroundAudios>, background_channel: Res<AudioChannel<BackgroundChannel>>) {
    if !background_channel.is_playing_sound() {
        background_channel.play(background_audios.background.clone())
            .looped()
            .with_volume(0.2);
    };
}