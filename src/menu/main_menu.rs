use bevy::prelude::*;
use crate::engine::GameState::MainMenu;

pub struct MainMenuPlugin;
#[derive(Component)]
struct MenuRoot;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(MainMenu), spawn_menu);
    app.add_systems(OnExit(MainMenu), despawn_menu);
  }
}

fn spawn_menu(mut commands: Commands) {
  commands.spawn((MenuRoot, Camera2d));

  commands.spawn((
    MenuRoot,
    Node {
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      ..default()
    },
    BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
  ));
}

fn despawn_menu(
  mut commands: Commands,
  query: Query<Entity, With<MenuRoot>>,
) {
  for entity in &query {
    commands.entity(entity).despawn();
  }
}
