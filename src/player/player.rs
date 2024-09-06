use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioSource as KiraAudioSource;

use crate::gravity::gravity::Gravity;
use crate::knife::knife::PlayerHitEvent;
use crate::movement::movement::{Body, Movement};
use crate::coin::coin::CoinTouchedEvent;
use crate::points::points::Points;
use crate::{CleanupGameStateExit, GameState};

use super::player_input::{InputDirection, MovementInputEvent};

const PLAYER_SPEED: f32 = 500.0;

pub struct PlayerPlugin<GameState: States> {
    pub state: GameState,
}

impl Plugin for PlayerPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_audio_channel::<PlayerChannel>();
        app.add_systems(OnEnter(GameState::Game), spawn_player
            .run_if(in_state(self.state.clone())));
        app.add_systems(Update, (animate_sprite, basic_state_machine, listen_movement_input, keep_player_in_bounds, listen_for_knives, listen_for_coins)
            .run_if(in_state(self.state.clone())));
        app.add_systems(Update, play_footsteps.run_if(in_state(self.state.clone())));
    }
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAnimationAssets {
    #[asset(texture_atlas_layout(tile_size_x = 16, tile_size_y = 16, columns = 4, rows = 1,))]
    layout: Handle<TextureAtlasLayout>,

    #[asset(image(sample = nearest))]
    #[asset(path = "sprites/skeleton_idle_animation.png")]
    idle: Handle<Image>,

    #[asset(image(sample = nearest))]
    #[asset(path = "sprites/skeleton_move_animation.png")]
    walking: Handle<Image>
}

#[derive(AssetCollection, Resource)]
pub struct PlayerAudioSources {
    #[asset(path = "audio/footsteps.mp3")]
    footsteps: Handle<KiraAudioSource>
}

#[derive(Resource)]
pub struct PlayerChannel;

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    movement: Movement,
    sprite: SpriteBundle,
    atlas: TextureAtlas,
    animation_timer: AnimationTimer
}

#[derive(Component)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub state: PlayerState
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash, States)]
pub enum PlayerState {
    Idle,
    Walking 
}

fn spawn_player(mut commands: Commands, animations: Res<PlayerAnimationAssets>) {
    
    commands.spawn(PlayerBundle {
        player: Player { speed: PLAYER_SPEED, state: PlayerState::Idle },
        sprite: SpriteBundle {
            texture: animations.idle.clone(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(16.0, 16.0)),
                ..default()
            },
            transform: Transform {
                scale: Vec3::splat(4.0),
                ..default()
            },
            ..default()
        },
        movement: Movement {
            gravity: Gravity {
                is_touching_terrain: false
            },
            body: Body {
                mass: 100.0,
                velocity: Vec2::ZERO,
                ..default()
            }
        },
        atlas: TextureAtlas::from(animations.layout.clone()),
        animation_timer: AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating))
    })
    .insert((Name::new("Player"), CleanupGameStateExit));
}

fn listen_movement_input(mut ev_movement: EventReader<MovementInputEvent>, mut movement_query: Query<(&mut Movement, &Player)>) {
    
    if let Ok((mut movement, player)) = movement_query.get_single_mut() {
        for input_direction in ev_movement.read() {
            match input_direction.direction {
                InputDirection::Left => movement.body.velocity.x = -player.speed,
                InputDirection::Right => movement.body.velocity.x = player.speed,
                InputDirection::None => movement.body.velocity.x = 0.0,
            }
        }
    }
}

fn listen_for_knives(mut ev_player_hit: EventReader<PlayerHitEvent>, mut game_state: ResMut<NextState<GameState>>) {
    for event in ev_player_hit.read() {
        game_state.set(GameState::Menu);
    }
}

fn listen_for_coins(mut ev_coin_collected: EventReader<CoinTouchedEvent>, mut points: ResMut<Points>) {
    for event in ev_coin_collected.read() {
        points.value += event.value;
    }
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut AnimationTimer, &mut TextureAtlas), With<Player>>) {
    for (mut timer, mut sprite) in &mut query  {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            sprite.index = (sprite.index + 1) % 4;
        }
    }
}

fn play_footsteps(audios: Res<PlayerAudioSources>, player_channel: Res<AudioChannel<PlayerChannel>>, player_query: Query<&Player>) {
    let player = player_query.get_single().unwrap();

    if player.state == PlayerState::Walking && !player_channel.is_playing_sound() {
        player_channel.play(audios.footsteps.clone()).looped().with_volume(0.5);
    }

    if player.state != PlayerState::Walking && player_channel.is_playing_sound() {
        player_channel.stop();
    }
}

fn basic_state_machine(mut query: Query<(&mut Player, &Movement, &mut AnimationTimer, &mut Handle<Image>, &mut TextureAtlas)>, animations: Res<PlayerAnimationAssets>) {
    for (mut player, movement, mut timer, mut sprite, mut atlas) in &mut query {
        
        if movement.body.velocity.x == 0.0 && player.state != PlayerState::Idle {
            player.state = PlayerState::Idle;
            *sprite = animations.idle.clone();
            *timer = AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating));
            *atlas = animations.layout.clone().into();
        }
    
        if movement.body.velocity.x != 0.0 && player.state != PlayerState::Walking {
            player.state = PlayerState::Walking;
            *sprite = animations.walking.clone();
            *timer = AnimationTimer(Timer::from_seconds(0.125, TimerMode::Repeating));
            *atlas = animations.layout.clone().into();
        }
    }
}

fn keep_player_in_bounds(window_query: Query<&Window, With<PrimaryWindow>>, mut query: Query<(&mut Transform, &Sprite), With<Player>>) {
    let window = window_query.get_single().unwrap();
    
    let half_width = window.width() / 2.0;
    let half_height = window.height() / 2.0;

    for (mut transform, sprite) in query.iter_mut() {
        let sprite_size = sprite.custom_size.unwrap_or(Vec2::ZERO) * transform.scale.truncate();

        transform.translation.x = transform.translation.x
            .clamp(-half_width + sprite_size.x / 2.0, half_width - sprite_size.x / 2.0);

        transform.translation.y = transform.translation.y
            .clamp(-half_height + sprite_size.y / 2.0, half_height - sprite_size.y / 2.0);
    }
}