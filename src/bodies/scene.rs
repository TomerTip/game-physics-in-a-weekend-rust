use bodies::body::Body;
use vec::vec::Vec3d;
use shape::shape::*;


struct Scence {
    bodies: Vec<dyn Body>
}

impl Scence {
    pub fn new(bodies: Vec<dyn Body>) -> Self {
        Scence {bodies: bodies}
    }

    pub fn init(&self) {
        // Spawn Object:
        let object: Body;
        object.shape = SphereShape::new(radius=1.0);
        object.position = Vec(0, 0, 5);
        object.orientation = Quat(0, 0, 0, 1);
        object.inv_mass = 1.0;
        self.bodies.push_back(object);

        // Spawn Ground:
        let ground: Body;
        ground.shape = BoxShape::new(1000, 1000, 1000); // A very large cube
        ground.position = Vec(0, 0, -1000); // The center of the cube, so that object will be spawn on top
        ground.orientation = Quat(0, 0, 0, 1);
        ground.inv_mass = 0;
        self.bodies.push_back(ground);
    }

    pub fn update(&mut self, dt_sec: f64) {
        for i in self.bodies.len() {
            let body : &mut Body = &mut self.bodies[i];

            //Gravirt needs to be an impulse
            // I = dp, F = dp/ d t => dp = F * dt => I = F * dt
            // F = m * g * sec 
            let mass = 1.0 / body.inv_mass;
            let gravity_vec = Vec3d(0, 0, -Body::GRAVITY);
            let impulse_gravity = gravity_vec * mass * dt_sec;
            body.apply_impulse_linear(impulse_gravity);
            self.bodies[i].linear_velocity += Vec3d(0, 0, -Body::GRAVITY) * dt_sec;
        }

        for i in self.bodies.len() {
            // Position update
            // dx = v * dt
            self.bodies[i].position += self.bodies[i].linear_velocity * dt_sec; 
        }
    }
}