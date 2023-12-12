use bodies::shape::*;
use vec::vec::Vec3d;
use quat::quat::Quat;

struct Body {
    shape: Box<dyn Shape>,
    orientation: Quat,
    position: Vec3d,
    inv_mass: f64, // In order to represent infinite mass, like a wall or earth
    linear_velocity: Vec3d
}

impl Body {
    pub const GRAVITY: f64 = 9.8; // m/s^2

    pub fn new(shape: Box<dyn Shape>, orientation: Vec3d, position: Quat) -> Self {
        Body {shape, orientation, position}
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
        let tmp: Vec3d = world - self.get_center_mass();
        let inverse_orient: Quat = self.orientation.inverse();
        let body_space: Vec3d = inverse_orient.rotate_point(tmp);
        return body_space
    }

    pub fn body_space_to_world_space(&self, world: &Vec3d) -> Vec3d {
        let center_mass_world_space: Vec3d = self.world_space_to_body_space();
        let world_space: Vec3d = self.orientation.rotate_point(world);
        return world_space
    }

    pub fn apply_impulse_linear(&mut self, impulse: &Vec3d) {
        if self.shape.get_center_mass() <= 0 {
            return;
        }
        // dv = J / m = dp / m
        self.linear_velocity = impulse * self.inv_mass;
    }
}