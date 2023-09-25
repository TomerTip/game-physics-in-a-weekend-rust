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

    pub fn zero(x: f64, y: f64, z: f64) -> Self {
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
        
        if mag > 0.0 {
            return Vec3d {
                x: self.x * inv_mag,
                y: self.y * inv_mag,
                z: self.z * inv_mag
            }
        }

        return Vec3d {
            x: self.x,
            y: self.y,
            z: self.z
        }
    }

    pub fn cross(self, other: Vec3d) -> Vec3d {
        // Vector A: A = (A₁, A₂, A₃)
        // Vector B: B = (B₁, B₂, B₃)
        // A × B = ((A₂ * B₃ - A₃ * B₂), (A₃ * B₁ - A₁ * B₃), (A₁ * B₂ - A₂ * B₁))

        Vec3d { x: (self.y * other.z) - (self.z * other.y),
                y: (self.z * other.x) - (self.x * other.z),
                z: (self.x * other.y) - (self.y * other.x)
        }
    }

    pub fn dot(&mut self, other: Vec3d) -> f64 {
        // Vector A: A = (A₁, A₂, A₃, ..., Aₙ)
        // Vector B: B = (B₁, B₂, B₃, ..., Bₙ)
        // A ⋅ B = A₁ * B₁ + A₂ * B₂ + A₃ * B₃ + ... + Aₙ * Bₙ

        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    pub fn get_ortho(self, u: &mut Vec3d, v: &mut Vec3d) {
        let n = self.normalize();

        let w = if n.z * n.z > 0.9 * 0.9 {
                Vec3d {x: 1.0, y: 0.0, z: 0.0}
            } else {
                Vec3d {x: 0.0, y: 0.0, z: 1.0}
            };

        *u = w.cross(n);
        *u = u.normalize();

        *v = w.cross(n);
        *v = v.normalize();
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
            z: self.z - other.z
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

// Implement the `Index` trait for your custom type.
impl std::ops::Index<usize> for Vec3d {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index out of bounds for Vec3d"),
        }
    }
}

