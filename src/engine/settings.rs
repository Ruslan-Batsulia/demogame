use std::fs;
use bevy::prelude::*;
use std::path::PathBuf;
use directories::UserDirs;
use super::schema::Settings;

#[derive(Resource)]
pub struct GameSettings(pub Settings);
pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
  fn build(&self, app: &mut App) {
    app.insert_resource(GameSettings(load_settings()));
  }
}

fn settings_path() -> Option<PathBuf> {
  let user_dirs = UserDirs::new()?;
  let documents = user_dirs.document_dir()?;

  return Some(documents.join("DemoGame").join("settings.toml"));
}

pub fn load_settings() -> Settings {
  let Some(path) = settings_path() else { return Settings::default() };

  if let Ok(content) = fs::read_to_string(&path) {
    let settings: Settings = toml::from_str(&content).unwrap_or_default();
    save_settings(&settings);

    return settings;
  }

  let defaults = Settings::default();
  save_settings(&defaults);
  return defaults;
}

pub fn save_settings(settings: &Settings) {
  let Some(path) = settings_path() else { return };

  if let Some(dir) = path.parent() {
    let _ = fs::create_dir_all(dir);
  }

  if let Ok(content) = toml::to_string_pretty(settings) {
    let _ = fs::write(&path, content);
  }
}
