#[derive(Debug, Clone, Copy)]
pub struct Vec4d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vec4d {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4d {x, y, z, w}
    }

    pub fn new_zero(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4d {x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.x.powi(2) + 
         self.y.powi(2) +
         self.z.powi(2) +
         self.w.powi(2)
        ).sqrt()
    }

    pub fn normalize(&self) -> Vec4d {
        // Normal is N = V / |V|
        // Each coordinate is divided by magnitute
        let mag = self.get_magnitude();
        let inv_mag = 1.0 / mag;
        
        if mag > 0.0 {
            return Vec4d {
                x: self.x * inv_mag,
                y: self.y * inv_mag,
                z: self.z * inv_mag,
                w: self.w * inv_mag
            }
        }

        return Vec4d {
            x: self.x,
            y: self.y,
            z: self.z,
            w: self.w
        }
    }

    pub fn dot(&mut self, other: Vec4d) -> f64 {
        // Vector A: A = (A₁, A₂, A₃, ..., Aₙ)
        // Vector B: B = (B₁, B₂, B₃, ..., Bₙ)
        // A ⋅ B = A₁ * B₁ + A₂ * B₂ + A₃ * B₃ + ... + Aₙ * Bₙ

        return  (self.x * other.x) +
                (self.y * other.y) +
                (self.z * other.z) +
                (self.w * other.w)
    }
}

impl std::ops::Add<Vec4d> for Vec4d {
    type Output = Vec4d;

    fn add(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}

impl std::ops::AddAssign<Vec4d> for Vec4d {
    fn add_assign(&mut self, other: Vec4d) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

impl std::ops::Mul<Vec4d> for Vec4d {
    type Output = Vec4d;

    fn mul(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}

impl std::ops::MulAssign<Vec4d> for Vec4d {
    fn mul_assign(&mut self, other: Vec4d) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
    }
}

impl std::ops::Sub<Vec4d> for Vec4d {
    type Output = Vec4d;

    fn sub(self, other: Vec4d) -> Vec4d {
        Vec4d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}

impl std::ops::SubAssign<Vec4d> for Vec4d {
    fn sub_assign(&mut self, other: Vec4d) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

impl std::ops::Div<f64> for Vec4d {
    type Output = Vec4d;

    fn div(self, scalar: f64) -> Vec4d {
        Vec4d {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
}

impl std::ops::DivAssign<f64> for Vec4d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
        self.w /= scalar;
    }
}

// Implement PartialEq trait for equality comparisons (== and !=)
impl PartialEq for Vec4d {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) &&
        (self.y == other.y) &&
        (self.z == other.z) &&
        (self.w == other.w)
    }
}

// Implement the `Index` trait for your custom type.
impl std::ops::Index<usize> for Vec4d {
    type Output = f64;

    fn index(&self, idx: usize) -> &f64 {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index out of bounds for Vec4d"),
        }
    }
}

