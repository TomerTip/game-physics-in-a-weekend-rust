
use bevy::prelude::*;
use bevy::prelude::shape as bevy_shape;

// mod physics::bodies::;
use crate::physics::{
    body,
    shape
};

use body::Body;
use body::ColorT;
use shape::ShapeT;


#[derive(Debug, Clone)]
pub struct BodyAdapter {
    pub my_body: Body,
    pub mesh: Mesh,
    pub material: StandardMaterial,
    pub transform: Transform
}

impl BodyAdapter {
    pub fn new(my_body: Body) -> Self {
        let my_body_shape = my_body.get_shape();

        let mesh: Mesh = match my_body_shape {   
            ShapeT::SphereShape { radius, .. } => {
                Mesh::from(bevy_shape::UVSphere {
                    radius: radius as f32,
                    sectors: 32,
                    stacks: 64 // Number of subdivision - (this value is arbitrary, and pretty much looks like a sphere)
                })
            },
            ShapeT::BoxShape { width, length, height, .. } => {
                Mesh::from(bevy_shape::Box::new (
                width as f32,
                length as f32,
                height as f32,
                ))
            }
        };

        let material: StandardMaterial = 
            match my_body.get_color() {
                ColorT::WHITE => Color::WHITE,
                ColorT::RED => Color::RED,
                ColorT::BLUE => Color::BLUE,
                ColorT::YELLOW => Color::YELLOW,
                ColorT::GREEN => Color::GREEN,
                _ => Color::WHITE
            }
        .into();

        // let material : StandardMaterial = Color::WHITE.into();

        let transform = Transform::from_xyz(
            my_body.position.x as f32,
            my_body.position.y as f32,
            my_body.position.z as f32
        );

        return BodyAdapter {
            my_body,
            mesh,
            material,
            transform
        }
    }

    pub fn get_mesh(self) -> Mesh {
        self.mesh
    }

    pub fn get_material(self) -> StandardMaterial {
        self.material
    }

    pub fn get_transform(self) -> Transform {
        self.transform
    }
}
