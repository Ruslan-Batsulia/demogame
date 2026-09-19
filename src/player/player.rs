mod movement;
mod components;
mod mouse_look;

use bevy::prelude::*;
pub use components::{Player, PlayerCamera};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (
      mouse_look::mouse_look,
      movement::move_player.after(mouse_look::mouse_look),
    ));
  }
}
