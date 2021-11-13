use bevy::prelude::App;
use shader_learning::AppEnvironmentPlugin;

fn main() {
    App::new()
        .add_plugin(AppEnvironmentPlugin)
        .run();
}
