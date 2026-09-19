use bevy::prelude::*;
use super::components::Player;

pub fn move_player(
  time: Res<Time>,
  keyboard: Res<ButtonInput<KeyCode>>,
  mut players: Query<(&mut Transform, &Player)>,
) {
  let Ok((mut player_transform, player)) = players.single_mut() else { return };

  let mut direction = Vec3::ZERO;

  let forward = player_transform.forward();
  let back = player_transform.back();
  let left = player_transform.left();
  let right = player_transform.right();

  if keyboard.pressed(KeyCode::KeyW) { direction += *forward; }
  if keyboard.pressed(KeyCode::KeyS) { direction += *back; }
  if keyboard.pressed(KeyCode::KeyA) { direction += *left; }
  if keyboard.pressed(KeyCode::KeyD) { direction += *right; }

  if direction != Vec3::ZERO { direction = direction.normalize(); }

  let speed = if keyboard.pressed(KeyCode::ShiftLeft) {
    player.sprint_speed
  } else {
    player.move_speed
  };

  player_transform.translation += direction * speed * time.delta_secs();
}
