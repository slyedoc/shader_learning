
use bevy::prelude::App;
use shader_learning::StandardEnviromentPlugin;

fn main() {
    App::new()
        .add_plugin(StandardEnviromentPlugin)
        .run();
}
