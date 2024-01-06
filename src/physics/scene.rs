use crate::physics::body::*;
use crate::physics::shape::*;
use crate::physics::vec::vec3d::Vec3d;
use crate::physics::quat::quat::Quat;

pub struct Scene {
    bodies: Vec<Body>
}

// impl Scene {
//     pub fn new(bodies: Vec<Body>) -> Self {
//         Scene {bodies: bodies}
//     }

//     pub fn init(&self) {
//         // Spawn Object:
//         let object: Body;
//         object.shape = ShapeT::new_sphere(1.0); 
//         object.position = Vec3d::new(0.0, 0.0, 5.0);
//         object.orientation = Quat::new(0.0, 0.0, 0.0, 1.0);
//         object.inv_mass = 1.0;
//         self.bodies.push(  object);

//         // Spawn Ground:
//         let ground: Body;
//         ground.shape = ShapeT::new_box(1000.0, 1000.0, 1000.0); // A very large cube
//         ground.position = Vec3d::new(0.0, 0.0, -1000.0); // The center of the cube, so that object will be spawn on top
//         ground.orientation = Quat::new(0.0, 0.0, 0.0, 1.0);
//         ground.inv_mass = 0.0;
//         self.bodies.push(ground);
//     }

//     pub fn update(&mut self, dt_sec: f64) {
//         for &mut body in &mut self.bodies {
//             //Gravity needs to be an impulse
//             // I = dp, F = dp/ d t => dp = F * dt => I = F * dt
//             // F = m * g * sec 
//             let mass = 1.0 / body.inv_mass;
//             let gravity_vec = Vec3d::new(0.0, 0.0, -Body::GRAVITY);
//             let impulse_gravity = gravity_vec * mass * dt_sec;
//             body.apply_impulse_linear(impulse_gravity);
//             body.linear_velocity += Vec3d::new(0.0, 0.0, -Body::GRAVITY) * dt_sec;
//         }

//         for &mut body in &mut self.bodies {
//             // Position update
//             // dx = v * dt
//             body.position += body.linear_velocity * dt_sec; 
//         }
//     }
// }