use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use super::components::{Player, PlayerCamera};

pub fn mouse_look(
  mut mouse_motion: MessageReader<MouseMotion>,
  mut players: Query<(&mut Transform, &mut Player), Without<PlayerCamera>>,
  mut cameras: Query<(&mut Transform, &mut PlayerCamera), Without<Player>>,
) {
  let Ok((mut player_transform, mut player)) = players.single_mut() else { return };
  let Ok((mut camera_transform, mut camera)) = cameras.single_mut() else { return };

  for event in mouse_motion.read() {
    player.yaw -= event.delta.x * player.mouse_sensitivity;
    camera.pitch -= event.delta.y * player.mouse_sensitivity;
  }

  camera.pitch = camera.pitch.clamp(-camera.pitch_limit, camera.pitch_limit);

  player_transform.rotation = Quat::from_rotation_y(player.yaw);
  camera_transform.rotation = Quat::from_rotation_x(camera.pitch);
}
