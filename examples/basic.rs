use bevy::{
    math::vec3,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_sub_color::{SubColorMaterial, SubColorMaterialPlugin};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(SubColorMaterialPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SubColorMaterial>>,
) {
    let quad = Mesh2dHandle(meshes.add(shape::Quad::default().into()));

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: quad.clone(),
        transform: Transform {
            translation: vec3(-25., 0., 0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(100.),
        },
        material: materials.add(Color::rgb(1., 1., 0.).into()),
        ..Default::default()
    });

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: quad.clone(),
        transform: Transform {
            translation: vec3(25., 25., 0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(100.),
        },
        material: materials.add(Color::rgb(1., 0., 1.).into())),
        ..Default::default()
    });

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: quad,
        transform: Transform {
            translation: vec3(0., -25., 0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(100.),
        },
        material: materials.add(Color::rgb(0., 1., 1.).into())),
        ..Default::default()
    });
}
