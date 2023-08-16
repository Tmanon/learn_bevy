use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Gravity;

mod arena;
mod players;

use crate::constants::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
            .add_systems(Startup, arena::spawn_arena)
            .add_systems(Startup, players::spawn_players);
    }
}
