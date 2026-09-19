use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
  #[default]
  InGame,
  MainMenu,
}
pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app.init_state::<GameState>();
  }
}
