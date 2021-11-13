use bevy::{
    app::AppExit,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    PipelinedDefaultPlugins,
};
use bevy_inspector_egui::{ WorldInspectorParams, WorldInspectorPlugin};
use prelude::CameraControllerPlugin;

#[allow(clippy::type_complexity)]
pub mod camera_controller;

pub mod prelude {
    pub use crate::{camera_controller::*, AppEnvironmentPlugin};
}

pub struct AppEnvironmentPlugin;

impl Plugin for AppEnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PipelinedDefaultPlugins)
            .add_plugin(WorldInspectorPlugin::default())
            .add_plugin(CameraControllerPlugin)
            //.add_plugin(FrameTimeDiagnosticsPlugin::default())
            //.add_plugin(LogDiagnosticsPlugin::default())
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
