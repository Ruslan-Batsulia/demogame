use bevy::{prelude::*, text::LineHeight};
use crate::engine::{GameState, MenuScreen};

pub struct MainMenuPlugin;

#[derive(Component)] struct MenuRoot;
#[derive(Component)] struct RootScreenRoot;
#[derive(Component)] struct PlayButton;
#[derive(Component)] struct SettingsButton;
#[derive(Component)] struct ExitButton;

impl Plugin for MainMenuPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), spawn_camera)
      .add_systems(OnExit(GameState::MainMenu), despawn_menu)
      .add_systems(OnEnter(MenuScreen::Root), spawn_root_screen)
      .add_systems(OnExit(MenuScreen::Root), despawn_root_screen)
      .add_systems(Update, button_visual.run_if(in_state(GameState::MainMenu)))
      .add_systems(Update, handle_settings_button.run_if(in_state(MenuScreen::Root)))
      .add_systems(Update, handle_exit_button.run_if(in_state(MenuScreen::Root)))
      .add_systems(Update, handle_play_button.run_if(in_state(MenuScreen::Root)));
  }
}

fn spawn_camera(mut commands: Commands) {
  commands.spawn((MenuRoot, Camera2d));
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

fn spawn_root_screen(mut commands: Commands) {
  commands.spawn((
    MenuRoot,
    RootScreenRoot,
    Node {
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      width: Val::Percent(100.0),
      height: Val::Percent(100.0),
      row_gap: Val::Px(5.0),
      ..default()
    },
    BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
    children![
      (PlayButton, menu_button("Play")),
      (SettingsButton, menu_button("Settings")),
      (ExitButton, menu_button("Exit")),
    ],
  ));
}

fn despawn_root_screen(
  mut commands: Commands,
  query: Query<Entity, With<RootScreenRoot>>,
) {
  for entity in &query { commands.entity(entity).despawn(); }
}

fn despawn_menu(
  mut commands: Commands,
  query: Query<Entity, With<MenuRoot>>,
) {
  for entity in &query { commands.entity(entity).despawn(); }
}

fn button_visual(
  mut query: Query<
    (&Interaction, &mut BackgroundColor),
    (Changed<Interaction>, With<Button>)
  >,
) {
  for (interaction, mut background) in &mut query {
    *background = match interaction {
      Interaction::Pressed => BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
      Interaction::Hovered => BackgroundColor(Color::srgb(0.25, 0.25, 0.25)),
      Interaction::None => BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
    }
  }
}

fn handle_exit_button(
  query: Query<&Interaction, (Changed<Interaction>, With<ExitButton>)>,
  mut exit: MessageWriter<AppExit>,
) {
  for interaction in &query {
    if *interaction == Interaction::Pressed {
      exit.write(AppExit::Success);
    }
  }
}

fn handle_settings_button(
  query: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>,
  mut next_screen: ResMut<NextState<MenuScreen>>,
) {
  for interaction in &query {
    if *interaction == Interaction::Pressed {
      next_screen.set(MenuScreen::Settings);
    }
  }
}

fn handle_play_button(
  query: Query<&Interaction, (Changed<Interaction>, With<PlayButton>)>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  for interaction in &query {
    if *interaction == Interaction::Pressed {
      next_state.set(GameState::InGame);
    }
  }
}
