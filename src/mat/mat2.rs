#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub rows: [Vec2d; 2]
}

impl Vec3d {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3d {x, y, z}
    }

    pub fn new_zero(x: f64, y: f64, z: f64) -> Self {
        Vec3d {x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn determinant() -> f64 {
        (self.rows[0].x * self.rows[1].y) -
        (self.rows[0].y * self.rows[1].x)
    }
}

impl std::ops::AddAssign<Vec3d> for Vec3d {
    fn add_assign(&mut self, other: Vec3d) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::MulAssign<Vec3d> for Vec3d {
    fn mul_assign(&mut self, other: Vec3d) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
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