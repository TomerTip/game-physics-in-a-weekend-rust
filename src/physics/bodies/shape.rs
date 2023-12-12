/************* ENUMS ****************/

enum shape_t {
    SHAPE_SPHERE,
    SHAPE_BOX
}

/************* STRUCTS ****************/

trait Shape {
    fn get_type(&self) -> shapt_t;
    fn get_volume(&self) -> f64;
}

struct SphereShape {
    radius : f64,
    cetner_mass: Vec3d
}
type sphere_t = SphereShape;


struct BoxShape {
    width : f64,
    length: f64,
    height: f64,
    cetner_mass: Vec3d
}
type box_t = BoxShape;


/************* IMPLEMENTS ****************/


impl SphereShape {
    pub fn new(radius: f64) -> Self {
        sphere_t {radius: radius, center_mass: Vec3d::zero() }
    }

    pub fn get_center_mass(&self) -> Vec3d {
        self.center_mass
    }
}

impl Shape for SphereShape {
    fn get_type(&self) -> shapt_t {
        shape_t::SHAPE_SPHERE
    }
}


impl BoxShape {
    pub fn new(width: f64, length: f64, height: f64) -> Self {
        let center_mass = Vec3d::new(width/2, length/2, height/2);
        sphere_t {
            width: width,
            length: length,
            height: height,
            center_mass: center_mass
        }
    }

    pub fn get_center_mass(&self) -> Vec3d {
        self.center_mass
    }
}

/************* TRAITS ****************/

impl Shape for SphereShape {
    fn get_type(&self) -> shapt_t {
        shape_t::SHAPE_BOX
    }

    fn get_volume(&self) -> f64 {
        return 4/3 * f64::consts::PI * f64::powf(self.radius, 3)
    }
}



impl Shape for BoxShape {
    fn get_type(&self) -> shapt_t {
        shape_t::SHAPE_BOX
    }

    fn get_volume(&self) -> f64 {
        return self.width * self.length * self.height
    }
}