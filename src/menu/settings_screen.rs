use bevy::{prelude::*, text::LineHeight};
use crate::engine::MenuScreen;

pub struct SettingsScreenPlugin;

#[derive(Component)] struct SettingsScreenRoot;
#[derive(Component)] struct BackButton;

impl Plugin for SettingsScreenPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(MenuScreen::Settings), spawn_settings_screen)
      .add_systems(OnExit(MenuScreen::Settings), despawn_settings_screen)
      .add_systems(Update, handle_back_btn.run_if(in_state(MenuScreen::Settings)));
  }
}

fn menu_button(label: &str) -> impl Bundle {(
  Button,
  Node {
    width: Val::Px(200.0),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    padding: UiRect::vertical(Val::Px(10.0)),
    ..default()
  },
  BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
  children![(
    Text::new(label.to_string()),
    TextFont { font_size: FontSize::Px(16.0), ..default() },
    TextColor(Color::WHITE),
    LineHeight::RelativeToFont(1.0),
  )],
)}

fn spawn_settings_screen(mut commands: Commands) {
  commands.spawn((
    SettingsScreenRoot,
    Node {
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      row_gap: Val::Px(5.0),
      ..default()
    },
    BackgroundColor(Color::srgb(0.1, 0.2, 0.1)),
    children![
      (BackButton, menu_button("Back")),
    ],
  ));
}

fn despawn_settings_screen(
  mut commands: Commands,
  query: Query<Entity, With<SettingsScreenRoot>>,
) {
  for entity in &query { commands.entity(entity).despawn(); }
}

fn handle_back_btn(
  query: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
  mut next_state: ResMut<NextState<MenuScreen>>,
) {
  for interaction in &query {
    if *interaction == Interaction::Pressed {
      next_state.set(MenuScreen::Root);
    }
  }
}
