mod path_config;
mod window_config;
use bevy::prelude::*;

pub use path_config::*;
pub use window_config::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PathConfig>()
            .add_plugin(WindowConfigPlugin);
    }
}
