use crate::physics::shape::Shape;
use crate::physics::quat::quat::Quat;
use crate::physics::vec::vec3d::Vec3d;
use crate::physics::shape::*;

#[derive(Debug, Clone, Copy)]
pub enum ColorT {
    WHITE,
    RED,
    BLUE,
    GREEN,
    YELLOW
}

#[derive(Debug, Clone, Copy)]
pub struct Body {
    pub shape: ShapeT,
    pub orientation: Quat,
    pub position: Vec3d,
    pub inv_mass: f64, // In order to represent infinite mass, like a wall or earth
    pub linear_velocity: Vec3d,
    pub color: ColorT
}

impl Body {
    pub const GRAVITY: f64 = 9.8; // m/s^2

    pub fn new(shape: ShapeT, orientation: Quat, position: Vec3d, mass: f64, velocity: Vec3d, color: ColorT) -> Self {
        Body {
            shape,
            orientation,
            position,
            inv_mass: 1.0 / mass,
            linear_velocity: velocity,
            color,
        }
    }

    pub fn new_shape(shape: ShapeT) -> Self {
        Body {
            shape: shape,
            orientation: Quat::zero(),
            position: Vec3d::zero(),
            inv_mass: 1.0,
            linear_velocity: Vec3d::zero(),
            color: ColorT::WHITE,
        }
    }

    pub fn unit() -> Self {
        Body {
            shape: ShapeT::new_sphere(1.0),
            orientation: Quat::zero(),
            position: Vec3d::zero(),
            inv_mass: 1.0,
            linear_velocity: Vec3d::zero(),
            color: ColorT::WHITE,
        }
    }

    pub fn get_center_of_mass_world_space(&self) -> Vec3d {
        let center_mass: Vec3d = self.shape.get_center_mass();
        let pos: Vec3d = self.position + self.orientation.rotate_point(center_mass);
        return pos
    }

    pub fn get_center_of_mass_model_space(&self) -> Vec3d {
        let center_mass: Vec3d = self.shape.get_center_mass();
        return center_mass
    }

    pub fn world_space_to_body_space(&self, world: &Vec3d) -> Vec3d {
        let tmp: Vec3d = *world - self.get_center_of_mass_world_space();
        let inverse_orient: Quat = self.orientation.inverse();
        let body_space: Vec3d = inverse_orient.rotate_point(tmp);
        return body_space
    }

    pub fn body_space_to_world_space(&self, world: &Vec3d) -> Vec3d {
        let center_mass_world_space: Vec3d = self.world_space_to_body_space(world);
        let world_space: Vec3d = center_mass_world_space + self.orientation.rotate_point(*world);
        return world_space
    }

    pub fn apply_impulse_linear(&mut self, impulse: Vec3d) {
        if self.inv_mass == 0.0 {
            return;
        }
        // dv = J / m = dp / m
        self.linear_velocity = impulse * self.inv_mass;
    }

    pub fn get_shape(self) -> ShapeT {
        self.shape
    }

    pub fn get_color(self) -> ColorT {
        self.color
    }
}