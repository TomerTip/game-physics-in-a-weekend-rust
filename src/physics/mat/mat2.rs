use crate::vec::vec2d::Vec2d;

#[derive(Debug, Clone, Copy)]
pub struct Mat2 {
    pub rows: [Vec2d; 2]
}

impl Mat2 {
    pub fn new(row0: Vec2d, row1: Vec2d) -> Self {
        Mat2 {rows: [row0, row1]}
    }

    pub fn zero() -> Self {
        Mat2 {rows: [Vec2d::zero(), Vec2d::zero()]}
    }

    pub fn determinant(self) -> f64 {
        return (self.rows[0].x * self.rows[1].y) -
               (self.rows[0].y * self.rows[1].x)
    }
}

impl std::ops::AddAssign<f64> for Mat2 {
    fn add_assign(&mut self, scalar: f64) {
        self.rows[0] += scalar;
        self.rows[1] += scalar;
    }
}

impl std::ops::AddAssign<Mat2> for Mat2 {
    fn add_assign(&mut self, other: Mat2) {
        self.rows[0] += other.rows[0];
        self.rows[1] += other.rows[1];
    }
}

impl std::ops::MulAssign<f64> for Mat2 {
    fn mul_assign(&mut self, scalar: f64) {
        self.rows[0] *= scalar;
        self.rows[1] *= scalar;
    }
}

impl std::ops::MulAssign<Mat2> for Mat2 {
    fn mul_assign(&mut self, other: Mat2) {
        self.rows[0] *= other.rows[0];
        self.rows[1] *= other.rows[1];
    }
}
