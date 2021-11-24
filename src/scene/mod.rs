use bevy::{
    core::Name,
    math::Quat,
    pbr2::*,
    prelude::{Assets, BuildChildren, Commands, ResMut, Transform},
    render2::{color::Color, mesh::*},
};

pub fn setup_color_testing_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground plane
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 10.0 })),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Name::new("ground"));

    // left wall
    let mut transform = Transform::from_xyz(2.5, 2.5, 0.0);
    transform.rotate(Quat::from_rotation_z(std::f32::consts::FRAC_PI_2));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Name::new("Wall - Left"));

    // back (right) wall
    let mut transform = Transform::from_xyz(0.0, 2.5, -2.5);
    transform.rotate(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2));
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(5.0, 0.15, 5.0))),
            transform,
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                perceptual_roughness: 1.0,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(Name::new("Wall - Right"));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::ORANGE_RED,
        brightness: 0.02,
    });

    // White point light
    commands
        .spawn_bundle(PointLightBundle {
            // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            transform: Transform::from_xyz(1.0, 2.0, 0.0),
            point_light: PointLight {
                intensity: 1600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                color: Color::WHITE,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Name::new("Point Light"))
        .with_children(|builder| {
            builder.spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.1,
                    ..Default::default()
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::WHITE,
                    emissive: Color::RED,
                    ..Default::default()
                }),
                ..Default::default()
            });
        });
}
