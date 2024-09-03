use::bevy::prelude::*;

use crate::{gravity::gravity::Gravity, movement::movement::{Body, Movement}, player::player::Player, CleanupGameStateExit, GameState};


pub struct CoinPlugin<GameState: States> {
    pub state: GameState
}

impl Plugin for CoinPlugin<GameState> {
    fn build(&self, app: &mut App) {
        app.add_event::<CoinTouchedEvent>();
        app.add_systems(Update, check_if_touch_player.run_if(in_state(self.state.clone())));
    }
}

const COIN_SPRITE_PATH: &str = "sprites/coin.png";

#[derive(Bundle)]
pub struct CoinBundle {
    coin: Coin,
    sprite: SpriteBundle,
    movement: Movement
}

#[derive(Component)]
pub struct Coin {
    pub value: i32
}

#[derive(Event)]
pub struct CoinTouchedEvent {
    pub value: i32
}

pub fn spawn_coin(mut commands: Commands, asset_server: Res<AssetServer>, spawn_position: Vec3) {
    commands.spawn(CoinBundle {
        coin: Coin { value: 10 },
        sprite: SpriteBundle {
            texture: asset_server.load(COIN_SPRITE_PATH),
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
            body: Body {
                mass: 100.0,
                velocity: Vec2::ZERO
            }
        }
    }).insert(CleanupGameStateExit);
}

fn check_if_touch_player(mut commands: Commands, mut ev_coin_collected: EventWriter<CoinTouchedEvent>, coin_query: Query<(&Movement, &Transform, &Sprite, &Coin, Entity)>, player_query: Query<(&Transform, &Sprite, &Player)>) {

    if let Ok((player_transform, player_sprite, _player)) = player_query.get_single() {
        let player_half_size = player_sprite.custom_size.unwrap() * player_transform.scale.truncate() / 2.0;

        for (_movement, transform, sprite, coin, entity) in coin_query.iter() {
            let coin_half_size = sprite.custom_size.unwrap() * transform.scale.truncate() / 2.0;
            
            let distance = transform.translation - player_transform.translation;

            if distance.x.abs() < coin_half_size.x + player_half_size.x &&
                       distance.y.abs() < coin_half_size.y + player_half_size.y {
                        ev_coin_collected.send(CoinTouchedEvent { value: coin.value });
                        commands.entity(entity).despawn();
                    }
        }
    }
}