#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "hud/hud.rs"] mod hud;
#[path = "menu/menu.rs"] mod menu;
#[path = "engine/engine.rs"] mod engine;
#[path = "player/player.rs"] mod player;
#[path = "levels/levels.rs"] mod levels;

use bevy::prelude::*;

use hud::HudPlugin;
use player::PlayerPlugin;
use menu::MainMenuPlugin;
use menu::SettingsScreenPlugin;
use levels::SaveHousePlugin;
use engine::StatePlugin;
use engine::SettingsPlugin;
use engine::WindowCorePlugin;
use engine::NavigationPlugin;

fn main() {
  App::new().add_plugins((
    SettingsPlugin,
    WindowCorePlugin,
    StatePlugin,
    MainMenuPlugin,
    SettingsScreenPlugin,
    NavigationPlugin,
    HudPlugin,
    PlayerPlugin,
    SaveHousePlugin,
  )).run();
}
