use::bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::{random, Rng};

use crate::GameState;

use super::knife::spawn_knife;

pub struct KnifeSpawnerPlugin<GameState: States> {
    pub state: GameState
}

const KNIFE_SPAWN_TIMER: f32 = 0.2;

impl Plugin for KnifeSpawnerPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.init_resource::<KnifeSpawnTimer>();
        app.add_systems(Update, (tick_knife_spawn_timer, spawn_knife_over_time).run_if(in_state(self.state.clone())));
    }
}

#[derive(Resource)]
pub struct KnifeSpawnTimer {
    pub timer: Timer
}

impl Default for KnifeSpawnTimer {
    fn default() -> KnifeSpawnTimer {
        KnifeSpawnTimer { timer: Timer::from_seconds(KNIFE_SPAWN_TIMER, TimerMode::Repeating) }
    }
}

fn tick_knife_spawn_timer(mut knife_spawn_timer: ResMut<KnifeSpawnTimer>, time: Res<Time>) {
    knife_spawn_timer.timer.tick(time.delta());
}

fn spawn_knife_over_time(commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>, knife_spawn_timer: Res<KnifeSpawnTimer>, asset_server: Res<AssetServer>) {
    if knife_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = rand::thread_rng().gen_range((-window.width()/2.0)+16.0..(window.width()/2.0)-16.0);
        let spawn_y = (window.height() / 2.0) - 16.0;

        spawn_knife(commands, asset_server, Vec3::new(random_x, spawn_y, 0.0));
    }
}