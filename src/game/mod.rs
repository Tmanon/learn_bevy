use bevy::prelude::*;

mod arena;
mod players;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, arena::spawn_arena)
            .add_systems(Startup, players::spawn_players);
    }
}
