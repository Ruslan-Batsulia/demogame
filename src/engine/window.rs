use bevy::prelude::*;
use bevy::window::{
  WindowMode,
  PresentMode,
  WindowPosition,
  MonitorSelection,
  WindowResolution,
};
use crate::engine::GameSettings;
use crate::engine::GameState::{MainMenu, InGame};
use bevy::window::{CursorGrabMode, CursorOptions};
use super::schema::WindowModeSetting;

pub struct WindowCorePlugin;

impl Plugin for WindowCorePlugin {
  fn build(&self, app: &mut App) {
    let vsync = app.world().resource::<GameSettings>().0.graphics.vsync;
    let window_mode = app.world().resource::<GameSettings>().0.graphics.mode;
    let resolution = app.world().resource::<GameSettings>().0.graphics.resolution;
    let present_mode = if vsync {
      PresentMode::AutoVsync
    } else {
      PresentMode::AutoNoVsync
    };
    let mode = match window_mode {
      WindowModeSetting::Windowed => WindowMode::Windowed,
      WindowModeSetting::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
      WindowModeSetting::Exclusive => WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
    };

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "DemoGame".into(),
        resolution: WindowResolution::new(resolution.0, resolution.1)
          .with_scale_factor_override(1.0),
        present_mode,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        mode,
        ..default()
      }),
      ..default()
    }))
    .add_systems(OnEnter(InGame), grab_cursor)
    .add_systems(OnEnter(MainMenu), release_cursor)
    .add_systems(Update, (apply_vsync_on_change, apply_mode_on_change));
  }
}

fn grab_cursor(
  mut window: Single<&mut Window>,
  mut cursor: Single<&mut CursorOptions>,
) {
  cursor.visible = false;
  cursor.grab_mode = CursorGrabMode::Locked;

  let center = Vec2::new(window.width() / 2.0, window.height() / 2.0);
  window.set_cursor_position(Some(center));
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
  cursor.visible = true;
  cursor.grab_mode = CursorGrabMode::None;
}

fn apply_vsync_on_change(
  settings: Res<GameSettings>,
  mut window: Single<&mut Window>,
) {
  if settings.is_changed() {
    window.present_mode = if settings.0.graphics.vsync {
      PresentMode::AutoVsync
    } else {
      PresentMode::AutoNoVsync
    };
  }
}

fn apply_mode_on_change(
  settings: Res<GameSettings>,
  mut window: Single<&mut Window>,
) {
  if settings.is_changed() {
    window.mode = match settings.0.graphics.mode {
      WindowModeSetting::Windowed => WindowMode::Windowed,
      WindowModeSetting::Borderless => WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
      WindowModeSetting::Exclusive => WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
    }
  }
}
