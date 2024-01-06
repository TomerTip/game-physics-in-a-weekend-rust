use crate::physics::vec::vec3d::Vec3d;

/************* ENUMS ****************/

#[derive(Debug, Clone, Copy)]
pub enum ShapeT {
    SphereShape {
        radius : f64,
        center_mass: Vec3d
    },
    BoxShape {
        width : f64,
        length: f64,
        height: f64,
        center_mass: Vec3d
    }
}

/************* IMPLEMENTS ****************/

pub trait ShapeConstructor {
    fn new_sphere(radius: f64) -> Self;
    fn new_box(width: f64, length: f64, height: f64) -> Self;
}

pub trait Shape {
    fn get_volume(&self) -> f64;
    fn get_center_mass(&self) -> Vec3d;
}


impl ShapeConstructor for ShapeT {
    fn new_sphere(radius: f64) -> Self {
        ShapeT::SphereShape {radius: radius, center_mass: Vec3d::zero() }
    }

    fn new_box(width: f64, length: f64, height: f64) -> Self {
        let center_mass = Vec3d::new(width/2.0, length/2.0, height/2.0);
        ShapeT::BoxShape { width, length, height, center_mass }
    }
}

impl Shape for ShapeT {
    fn get_volume(&self) -> f64 {
        match self {
            ShapeT::BoxShape { width, length, height, .. } =>
                width * length * height,

            ShapeT::SphereShape { radius, .. } =>
                (4.0/3.0) * std::f64::consts::PI * f64::powf(*radius, 3.0),
        }
    }

    fn get_center_mass(&self) -> Vec3d {
        match self {
            ShapeT::SphereShape { center_mass, .. } => *center_mass,
            ShapeT::BoxShape { center_mass, .. } => *center_mass,
        }
    }
}