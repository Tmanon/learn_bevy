use bevy::prelude::*;

mod menu_bundles;
mod mouse;
mod radial_menu;
mod rect_menu;

pub struct MenuPlugin;
use crate::menu::mouse::*;
use crate::menu::radial_menu::radial_menu;
use crate::menu::rect_menu::*;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, radial_menu)
            .add_systems(Update, rect_menu_setup)
            .add_systems(Update, button_system)
            .add_systems(Update, mouse_system);
    }
}
