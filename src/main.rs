//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::prelude::*;
use physics::shape::ShapeConstructor;

mod physics;
mod visuals;

// mod physics::bodies::;
use crate::physics::{
    body,
    scene,
    shape,
    vec::{
       vec2d,
       vec3d,
       vec4d 
    },
    mat::{
        mat2,
        mat3,
        mat4
    },
    quat::quat
};
use body::Body as myBody;
use body::ColorT;

use shape::ShapeT;

use scene::Scene as myScene;

use vec3d::Vec3d as myVec3;
use quat::Quat as myQuat;

use crate::visuals::adapters;
use adapters::BodyAdapter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground = myBody::new(
        ShapeT::new_box(10.0, 0.01, 10.0),
        myQuat::zero(),
        myVec3::new(0.0, -2.0, 0.0),
        1000.0,
        myVec3::zero(),
        ColorT::WHITE
    );
    let ground_bundle = BodyAdapter::new(ground);

    let my_sphere = myBody::new(
        ShapeT::new_sphere(1.0),
        myQuat::zero(),
        myVec3::zero(),
        1.0,
        myVec3::zero(),
        ColorT::RED
    );
    let sphere_bundle = BodyAdapter::new(my_sphere);

    
    // box base
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            ground_bundle.clone().get_mesh().into()
        ),
        material: materials.add(
            ground_bundle.clone().get_material().into()
        ),
        transform: ground_bundle.clone().get_transform(),
        ..default()
    });
    // sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            sphere_bundle.clone().get_mesh().into()
        ),
        material: materials.add(
            sphere_bundle.clone().get_material().into()
        ),
        transform: sphere_bundle.clone().get_transform(),
        ..default()
    });
    
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}