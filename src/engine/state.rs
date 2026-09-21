use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
  #[default]
  MainMenu,
  InGame,
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum MenuScreen {
  #[default]
  Root,
  Settings,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
  fn build(&self, app: &mut App) {
    app.init_state::<GameState>()
      .init_state::<MenuScreen>();
  }
}
