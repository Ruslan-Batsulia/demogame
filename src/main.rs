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
use levels::SaveHousePlugin;
use engine::{
  StatePlugin,
  SettingsPlugin,
  WindowCorePlugin,
};

fn main() {
  App::new()
    .add_plugins(SettingsPlugin)
    .add_plugins(WindowCorePlugin)
    .add_plugins(StatePlugin)
    .add_plugins(MainMenuPlugin)
    .add_plugins(HudPlugin)
    .add_plugins(PlayerPlugin)
    .add_plugins(SaveHousePlugin)
    .run();
}
