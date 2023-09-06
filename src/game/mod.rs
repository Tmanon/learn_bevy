use bevy::prelude::*;
use bevy_xpbd_2d::prelude::Gravity;

mod arena;
mod constants;
mod game;
mod players;
mod show_keybindings;

use crate::game::constants::*;
use crate::states::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                game::game_setup,
                arena::spawn_arena,
                show_keybindings::show_keybindings_system,
                players::players,
            ),
        )
        .insert_resource(Gravity(Vec2::NEG_Y * GRAVITY))
        .add_systems(Update, game::in_game.run_if(in_state(AppState::InGame)));
    }
}
