use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
  pub yaw: f32, // Поворот вліво-вправо у радіанах.
  pub mouse_sensitivity: f32, // Множник чутливості миші.
  pub move_speed: f32, // Швидкість ходьби
  pub sprint_speed: f32, // Швидкість бігу.
}

#[derive(Component)]
pub struct PlayerCamera {
  pub pitch: f32, // Нахил вгору-вниз у радіанах.
  pub pitch_limit: f32, // Максимальний кут нахилу в радіанах.
}
