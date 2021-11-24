#![allow(clippy::type_complexity)]

mod config;
mod camera_controller;
mod scene;
mod shapes;

use bevy::{
    app::AppExit,
    prelude::*,
    PipelinedDefaultPlugins, diagnostic::{LogDiagnosticsPlugin, FrameTimeDiagnosticsPlugin},
};

use bevy_inspector_egui::{ WorldInspectorParams, WorldInspectorPlugin};
use camera_controller::CameraControllerPlugin;
use config::ConfigPlugin;
use shapes::ShapePlugin;

pub mod prelude {
    pub use crate::{camera_controller::*, scene::*, shapes::*, StandardEnviromentPlugin};
}

pub struct StandardEnviromentPlugin;

impl Plugin for StandardEnviromentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(WindowDescriptor {
                title: "Shader".to_string(),
                vsync: false, // disable to break 60 fps
                ..Default::default()
            })
            .add_plugins(PipelinedDefaultPlugins)
            .add_plugin(WorldInspectorPlugin::default())
            .add_plugin(CameraControllerPlugin)
            .add_plugin(ConfigPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default())
            .add_system(control_system);

    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn control_system(
    mut exit: EventWriter<AppExit>,
    key_input: Res<Input<KeyCode>>,
    mut world_inspection: ResMut<WorldInspectorParams>,
) {
    if key_input.just_pressed(KeyCode::F12) {
        world_inspection.enabled = !world_inspection.enabled;
    }
    if key_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
