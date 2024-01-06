use crate::physics::vec::vec3d::Vec3d;
use crate::physics::mat::mat3::Mat3;


#[derive(Debug, Clone, Copy)]
pub struct Quat {
    pub x : f64,
    pub y : f64,
    pub z : f64,
    pub w : f64
}

impl Quat {
    pub fn new(x : f64, y: f64, z : f64, w : f64) -> Self {
        Quat {x, y, z, w}
    }

    pub fn zero() -> Self {
        Quat {x: 0.0, y: 0.0, z: 0.0, w: 1.0}
    }

    pub fn from_axis_angle(mut vec: Vec3d, angle_radians: f64) -> Self {
        // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation

        let half_angle_radians : f64 = 0.5 * angle_radians;
        let half_cos : f64 = f64::cos(half_angle_radians);
        let half_sin: f64 = f64::sin(half_angle_radians);

        vec = vec.normalize();
    
        Quat {x: vec.x * half_sin,
              y: vec.y * half_sin,
              z: vec.z * half_sin,
              w: half_cos}
    }

    pub fn magnitute_squared(self) -> f64 {
        return (self.x * self.x) +
               (self.y * self.y) +
               (self.z * self.z) +
               (self.w * self.w);
    }

    pub fn get_magnitude(self) -> f64 {
        return self.magnitute_squared().sqrt();
    }

    pub fn normalize(&self) -> Quat {
        // Normal is N = V / |V|
        // Each coordinate is divided by magnitute
        let mag = self.get_magnitude();
        let inv_mag = 1.0 / mag;
        
        if mag > 0.0 {
            return Quat {
                x: self.x * inv_mag,
                y: self.y * inv_mag,
                z: self.z * inv_mag,
                w: self.x * inv_mag
            }
        }

        return Quat {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }

    pub fn inverse(self) -> Quat {
        let mut temp = self;
        temp *= 1.0 / self.magnitute_squared();
        temp.x = -temp.x;
        temp.y = -temp.y;
        temp.z = -temp.z;
        return temp;
    }

    pub fn rotate_point(self, vec : Vec3d) -> Vec3d {
        let q_vec : Quat = Quat::new(vec.x, vec.y, vec.z, 0.0);
        let rotated : Quat = self * q_vec * self.inverse();
        return Vec3d {x: rotated.x, y: rotated.y, z: rotated.z}
    }

    pub fn rotate_matrix(self, mat : Mat3) -> Mat3 {
       Mat3 {rows: [
        self.rotate_point(mat.rows[0]),
        self.rotate_point(mat.rows[1]),
        self.rotate_point(mat.rows[2])
       ]}
    }
}

impl std::ops::Mul<Quat> for Quat {
    type Output = Quat;

    fn mul(self, other: Quat) -> Quat {
        Quat {
            x: (self.x * other.w) + (self.w * other.x) - (self.y * other.z) - (self.z * other.y),
            y: (self.y * other.w) + (self.w * other.y) - (self.z * other.x) - (self.z * other.z),
            z: (self.z * other.w) + (self.w * other.z) - (self.x * other.y) - (self.z * other.x),
            w: (self.x * other.w) - (self.x * other.x) - (self.y * other.y) - (self.z * other.z)
        }
    }
}

impl std::ops::MulAssign<Quat> for Quat {
    fn mul_assign(&mut self, other: Quat) {
        *self = *self * other;
    }
}

impl std::ops::MulAssign<f64> for Quat {
    fn mul_assign(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
    }
}

// Implement PartialEq trait for equality comparisons (== and !=)
impl PartialEq for Quat {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z) &&
        (self.w == other.w)
    }
}

// Implement the `Index` trait for your custom type.
impl std::ops::Index<usize> for Quat {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Quat"),
        }
    }
}

impl std::ops::IndexMut<usize> for Quat {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index out of bounds for Quat"),
        }
    }
}

impl std::ops::Index<u64> for Quat {
    type Output = f64;

    fn index(&self, idx: u64) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Quat"),
        }
    }
}

impl std::ops::Index<i32> for Quat {
    type Output = f64;

    fn index(&self, idx: i32) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Quat"),
        }
    }
}