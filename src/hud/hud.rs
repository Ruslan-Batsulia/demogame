mod metrics;

use bevy::prelude::*;
use metrics::HudMetricsPlugin;

pub struct HudPlugin;

impl Plugin for HudPlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(HudMetricsPlugin);
  }
}
