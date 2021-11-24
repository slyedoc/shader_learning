use bevy::{
    core::Name,
    ecs::prelude::*,
    prelude::{App, Assets, GlobalTransform, Transform},
    render2::{
        camera::PerspectiveCameraBundle,
        color::Color,
        mesh::Mesh,
        view::{ComputedVisibility, Visibility},
    },
};
use shader_learning::prelude::*;

fn main() {
    App::new()
        .add_plugin(StandardEnviromentPlugin)
        //.add_startup_system(setup_color_testing_scene)
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomPaintingMaterial>>,
) {
    // cube
    commands.spawn().insert_bundle((
        ShapeInstance {
            value: Shape::Quad(Quad::default())
        },
        Transform::from_xyz(0.0, 0.5, 0.0),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        materials.add(CustomPaintingMaterial {
            color1: Color::GREEN,
            color2: Color::RED,
        }),
        Name::new("Quad"),
    ));

    // camera
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0.0, 0.5, 1.0),
            ..Default::default()
        })
        .insert(Name::new("Camera"))
        .insert(CameraController::default());
}
