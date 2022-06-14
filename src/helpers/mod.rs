mod camera_controller;
mod exit;
mod overlay;

use bevy::prelude::*;
//use bevy_inspector_egui::{WorldInspectorParams, WorldInspectorPlugin};
pub use camera_controller::*;
pub use exit::*;
pub use overlay::*;

pub struct HelperPlugin;

impl Plugin for HelperPlugin {
    fn build(&self, app: &mut App) {
        app
            // .insert_resource(WorldInspectorParams {
            //     enabled: false,
            //     ..Default::default()
            // })
            // .add_plugin(WorldInspectorPlugin::new())
            // Quality of life plugins
            .add_plugin(CameraControllerPlugin)
            .add_plugin(OverlayPlugin)
            .add_plugin(ExitPlugin);
            // Simple 3d cursor to test bvh
            //.add_system(HelperPlugin::toggle_inspector);
    }
}

// impl HelperPlugin {
//     fn toggle_inspector(
//         input: ResMut<Input<KeyCode>>,
//         mut window_params: ResMut<WorldInspectorParams>,
//     ) {
//         if input.just_pressed(KeyCode::Grave) {
//             window_params.enabled = !window_params.enabled
//         }
//     }
// }

// Adding a few system here to make reusing them easy

#[allow(dead_code)]
pub fn load_enviroment(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // light
    commands.spawn_bundle(DirectionalLightBundle {
        transform: Transform::from_xyz(50.0, 50.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    //ground
    commands
        .spawn_bundle(PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::DARK_GREEN,
                ..default()
            }),
            ..default()
        })
        .insert(Name::new("Ground"));
        
}

#[allow(dead_code)]
pub fn setup_cameras(mut commands: Commands) {
    //commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(CameraController::default());
        //.insert(BvhCamera::new(1024, 1024));
}
