mod movement;
mod components;
mod mouse_look;

use bevy::prelude::*;
pub use components::{Player, PlayerCamera};

use crate::engine::GameState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (
      mouse_look::mouse_look,
      movement::move_player.after(mouse_look::mouse_look),
    ).run_if(in_state(GameState::InGame)));
  }
}
