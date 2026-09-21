use bevy::prelude::*;
use crate::engine::MenuScreen;

use super::GameState;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, handle_escape.run_if(in_state(GameState::InGame)));
  }
}

fn handle_escape(
  keyboard: Res<ButtonInput<KeyCode>>,
  mut next_game_state: ResMut<NextState<GameState>>,
  mut next_menu_screen: ResMut<NextState<MenuScreen>>,
) {
  if keyboard.just_pressed(KeyCode::Escape) {
    next_game_state.set(GameState::MainMenu);
    next_menu_screen.set(MenuScreen::Root);
  }
}
