#![allow(warnings)]
mod helpers;
use helpers::*;
use shader_learning::*;
use bevy::prelude::*;

fn main() {
    let _app = App::new()
        .insert_resource(WindowDescriptor {
            title: "Shader Learning".to_string(),
            width: 400.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(HelperPlugin)
        .add_plugin(ShaderPlugin)
        .add_startup_system(helpers::setup_cameras)
        .add_startup_system(helpers::load_enviroment)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // cube
    commands.spawn().insert_bundle((
        meshes.add(Mesh::from(shape::Cube { size: 3.0 })),
        Transform::from_xyz(0.0, 1.5, 0.0),
        GlobalTransform::default(),
        //RandomMaterial,
        Visibility::default(),
        ComputedVisibility::default(),
    ));

}
