use::bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::{AudioApp, AudioChannel, AudioControl, AudioSource as KiraAudioSource};

use crate::{gravity::gravity::Gravity, movement::movement::{Body, Movement}, player::player::Player, CleanupGameStateExit, GameState};

pub struct CoinPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for CoinPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_event::<CoinTouchedEvent>();
        app.add_audio_channel::<CoinChannel>();
        app.add_systems(Update, (check_if_touch_player, animate_sprite).run_if(in_state(self.state.clone())));
    }
}

#[derive(AssetCollection, Resource)]
pub struct CoinAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1,))]
    layout: Handle<TextureAtlasLayout>,

    #[asset(image(sample = nearest))]
    #[asset(path = "sprites/coin_animation.png")]
    idle: Handle<Image>,

    #[asset(path = "audio/coin_collected.mp3")]
    coin_collected_sound: Handle<KiraAudioSource>
}

#[derive(Resource)]
pub struct CoinChannel;

#[derive(Bundle)]
pub struct CoinBundle {
    coin: Coin,
    movement: Movement,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    animation_timer: AnimationTimer
}

#[derive(Component)]
pub struct Coin {
    pub value: i32
}

#[derive(Event)]
pub struct CoinTouchedEvent {
    pub value: i32
}

#[derive(Component)]
struct AnimationTimer(Timer);

pub fn spawn_coin(mut commands: Commands, animations: Res<CoinAssets>, spawn_position: Vec3) {
    commands.spawn(CoinBundle {
        coin: Coin { value: 10 },
        sprite: SpriteBundle {
            texture: animations.idle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::splat(4.0),
                translation: spawn_position,
                ..default()
            },
            ..default()
        },
        movement: Movement {
            gravity: Gravity {
                is_touching_terrain: false
            },
            body: Body::default()
        },
        atlas: TextureAtlas::from(animations.layout.clone()),
        animation_timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating))
    }).insert(CleanupGameStateExit);
}

fn check_if_touch_player(mut commands: Commands, mut ev_coin_collected: EventWriter<CoinTouchedEvent>, coin_query: Query<(&Movement, &Transform, &Sprite, &Coin, Entity)>, player_query: Query<(&Transform, &Sprite, &Player)>,
    coin_assets: Res<CoinAssets>, coin_channel: Res<AudioChannel<CoinChannel>>) {

    if let Ok((player_transform, player_sprite, _player)) = player_query.get_single() {
        let player_half_size = player_sprite.custom_size.unwrap() * player_transform.scale.truncate() / 2.0;

        for (_movement, transform, sprite, coin, entity) in coin_query.iter() {
            let coin_half_size = sprite.custom_size.unwrap() * transform.scale.truncate() / 2.0;
            
            let distance = transform.translation - player_transform.translation;

            if distance.x.abs() < coin_half_size.x + player_half_size.x &&
                       distance.y.abs() < coin_half_size.y + player_half_size.y {
                        ev_coin_collected.send(CoinTouchedEvent { value: coin.value });
                        coin_channel.play(coin_assets.coin_collected_sound.clone()).with_volume(0.12);
                        commands.entity(entity).despawn();
                    }
        }
    }
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Coin>>) {
    for (mut timer, mut sprite) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}