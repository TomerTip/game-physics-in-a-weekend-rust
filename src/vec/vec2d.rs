#[derive(Debug, Clone, Copy)]
pub struct Vec2d {
    pub x: f64,
    pub y: f64,
}

impl Vec2d {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2d {x, y}
    }

    pub fn get_magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec2d {
        // Normal is N = V / |V|
        // Each coordinate is divided by magnitute
        let mag = self.get_magnitude();
        let inv_mag = 1.0 / mag;
        return Vec2d { x: self.x * inv_mag, y: self.y * inv_mag}
    }
}

impl std::ops::Add<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn add(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::AddAssign<Vec2d> for Vec2d {
    fn add_assign(&mut self, other: Vec2d) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::Mul<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn mul(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl std::ops::MulAssign<Vec2d> for Vec2d {
    fn mul_assign(&mut self, other: Vec2d) {
        self.x *= other.x;
        self.y *= other.y;
    }
}

impl std::ops::Sub<Vec2d> for Vec2d {
    type Output = Vec2d;

    fn sub(self, other: Vec2d) -> Vec2d {
        Vec2d {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::SubAssign<Vec2d> for Vec2d {
    fn sub_assign(&mut self, other: Vec2d) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl std::ops::Div<f64> for Vec2d {
    type Output = Vec2d;

    fn div(self, scalar: f64) -> Vec2d {
        Vec2d {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl std::ops::DivAssign<f64> for Vec2d {
    fn div_assign(&mut self, scalar: f64) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

// Implement PartialEq trait for equality comparisons (== and !=)
impl PartialEq for Vec2d {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}