use bevy::prelude::*;
use crate::engine::GameSettings;
use crate::engine::GameState::InGame;
use crate::player::{Player, PlayerCamera};

pub struct SaveHousePlugin;
#[derive(Component)]
struct LevelEntity;

impl Plugin for SaveHousePlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(InGame), spawn_level);
    app.add_systems(OnExit(InGame), despawn_level);
  }
}

fn spawn_level(
  mut commands: Commands,
  settings: Res<GameSettings>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let player = commands.spawn((
    Player {
      yaw: 0.0,
      mouse_sensitivity: settings.0.controls.mouse_sensitivity,
      move_speed: 2.0,
      sprint_speed: 4.0,
    },
    LevelEntity,
    Mesh3d(meshes.add(Capsule3d::new(0.5, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb(0.8, 0.8, 0.8))),
    Transform::from_xyz(2.0, 1.0, 4.0),
  )).id();

  commands.spawn((
    PlayerCamera {
      pitch: 0.0,
      pitch_limit: 89.9_f32.to_radians(),
    },
    LevelEntity,
    Camera3d::default(),
    Transform::from_xyz(0.0, 0.8, 0.0),
    ChildOf(player),
  ));

  commands.spawn((
    LevelEntity,
    Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
    MeshMaterial3d(materials.add(Color::srgb(0.3, 0.8, 0.3))),
    Transform::from_xyz(0.0, 0.5, 0.0),
  ));

  commands.spawn((
    LevelEntity,
    Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
    MeshMaterial3d(materials.add(Color::srgb(0.4, 0.4, 0.4))),
  ));
}

fn despawn_level(
  mut commands: Commands,
  query: Query<Entity, With<LevelEntity>>,
) {
  for entity in &query {
    commands.entity(entity).despawn();
  }
}
