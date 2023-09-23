#[derive(Debug, Clone, Copy)]
pub struct Vec3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d {x, y, z}
    }

    pub fn new_zero(x: f64, y: f64) -> Self {
        Vec3d {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.x.powi(2) + 
         self.y.powi(2) +
         self.z.powi(2)
        ).sqrt()
    }

    pub fn normalize(&self) -> Vec3d {
        // Normal is N = V / |V|
        // Each coordinate is divided by magnitute
        let mag = self.get_magnitude();
        let inv_mag = 1.0 / mag;
        return Vec3d {
            x: self.x * inv_mag,
            y: self.y * inv_mag,
            z: self.z * inv_mag
        }
    }
}

impl std::ops::Add<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn add(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::AddAssign<Vec3d> for Vec3d {
    fn add_assign(&mut self, other: Vec3d) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::Mul<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn mul(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl std::ops::MulAssign<Vec3d> for Vec3d {
    fn mul_assign(&mut self, other: Vec3d) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl std::ops::Sub<Vec3d> for Vec3d {
    type Output = Vec3d;

    fn sub(self, other: Vec3d) -> Vec3d {
        Vec3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.y - other.z
        }
    }
}

impl std::ops::SubAssign<Vec3d> for Vec3d {
    fn sub_assign(&mut self, other: Vec3d) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::Div<f64> for Vec3d {
    type Output = Vec3d;

    fn div(self, scalar: f64) -> Vec3d {
        Vec3d {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::ops::DivAssign<f64> for Vec3d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

// Implement PartialEq trait for equality comparisons (== and !=)
impl PartialEq for Vec3d {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z)
    }
}