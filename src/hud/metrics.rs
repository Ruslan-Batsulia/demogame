use bevy::prelude::*;
use crate::engine::GameState::InGame;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore};

#[derive(Component)]
struct FpsText;

pub struct HudMetricsPlugin;

impl Plugin for HudMetricsPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin::default())
      .add_systems(OnEnter(InGame), spawn_fps_text)
      .add_systems(OnExit(InGame), despawn_fps_text)
      .add_systems(Update, update_fps_text.run_if(in_state(InGame)));
  }
}

fn spawn_fps_text(mut commands: Commands) {
  commands.spawn((
    FpsText,
    Text::new("FPS: 0"),
    TextFont {
      font_size: FontSize::Px(10.0),
      ..default()
    },
    TextColor(Color::srgb(1.0, 1.0, 1.0)),
    Node {
      position_type: PositionType::Absolute,
      top: Val::Px(5.0),
      left: Val::Px(5.0),
      ..default()
    },
  ));
}

fn despawn_fps_text(
  mut commands: Commands,
  query: Query<Entity, With<FpsText>>,
) {
  for entity in &query {
    commands.entity(entity).despawn();
  }
}

fn update_fps_text(
  diagnostics: Res<DiagnosticsStore>,
  mut query: Query<&mut Text, With<FpsText>>,
  time: Res<Time>,
  mut timer: Local<f32>,
) {
  *timer += time.delta_secs();

  if *timer >= 1.0 {
    *timer = 0.0;

    let Ok(mut text) = query.single_mut() else { return };

    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
      if let Some(value) = fps.smoothed() {
        **text = format!("FPS: {}", value as u32);
      }
    }
  }
}
