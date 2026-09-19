use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Settings {
  #[serde(default)] pub graphics: GraphicsSettings,
  #[serde(default)] pub controls: ControlsSettings,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum WindowModeSetting {
  Windowed,
  Borderless,
  Exclusive,
}

#[derive(Serialize, Deserialize)]
pub struct GraphicsSettings {
  #[serde(default = "vsync")] pub vsync: bool,
  #[serde(default = "resolution")] pub resolution: (u32, u32),
  #[serde(default = "mode")] pub mode: WindowModeSetting,
}

#[derive(Serialize, Deserialize)]
pub struct ControlsSettings {
  #[serde(default = "mouse_sensitivity")] pub mouse_sensitivity: f32,
}

impl Default for Settings {
  fn default() -> Self {
    return Self {
      graphics: GraphicsSettings::default(),
      controls: ControlsSettings::default(),
    };
  }
}

impl Default for GraphicsSettings {
  fn default() -> Self {
    return Self {
      vsync: vsync(),
      resolution: resolution(),
      mode: mode(),
    }
  }
}

impl Default for ControlsSettings {
  fn default() -> Self {
    return Self {
      mouse_sensitivity: mouse_sensitivity(),
    }
  }
}

fn vsync() -> bool { return true; }
fn resolution() -> (u32, u32) { return (1920, 1080); }
fn mode() -> WindowModeSetting { return WindowModeSetting::Borderless; }
fn mouse_sensitivity() -> f32 { return 0.001; }
