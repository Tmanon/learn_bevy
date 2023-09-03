use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Gravity;

mod arena;
mod constants;
mod game;
mod players;
mod show_keybindings;

use crate::game::constants::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
            .add_systems(Startup, game::game)
            .add_systems(Startup, arena::spawn_arena)
            .add_systems(Startup, show_keybindings::show_keybindings_system)
            .add_systems(Startup, players::players);
    }
}
