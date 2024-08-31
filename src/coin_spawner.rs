use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::coin::spawn_coin;

pub struct CoinSpawnerPlugin;

const COIN_SPAWN_TIMER: f32 = 3.0;

impl Plugin for CoinSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CoinSpawnTimer>();
        app.add_systems(Update, (tick_coin_spawn_timer, spawn_coin_over_time));
    }
}

#[derive(Resource)]
pub struct CoinSpawnTimer {
    pub timer: Timer
}

impl Default for CoinSpawnTimer {
    fn default() -> Self {
        Self { timer: Timer::from_seconds(COIN_SPAWN_TIMER, TimerMode::Repeating) }
    }
}

fn tick_coin_spawn_timer(mut coin_spawn_timer: ResMut<CoinSpawnTimer>, time: Res<Time>) {
    coin_spawn_timer.timer.tick(time.delta());
}

fn spawn_coin_over_time(commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, coin_spawn_timer: Res<CoinSpawnTimer>, asset_server: Res<AssetServer>) {
    if coin_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = rand::thread_rng().gen_range((-window.width()/2.0)+16.0..(window.width()/2.0)-16.0);
        let spawn_y = (window.height() / 2.0) - 16.0;

        spawn_coin(commands, asset_server, Vec3::new(random_x, spawn_y, 0.0))
    }
}