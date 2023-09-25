#[derive(Debug, Clone, Copy)]
pub struct Mat3 {
    pub rows: [Vec3d; 3]
}

impl Mat3 {
    pub fn new(row0: Vec3d, row1: Vec3d, row2: Vec3d) -> Self {
        Mat3 {rows: [row0, row1, row2]}
    }

    pub fn zero() -> Self {
        Mat3 {rows: [Vec3d::zero(), Vec3d::zero(), Vec3d::zero()]}
    }

    pub fn identity() {
        Mat3{ Vec3d(1, 0, 0),
              Vec3d(0, 1, 0),
              Vec3d(0, 0, 1)
            }
    }

    pub fn trace(&self) -> f64{
        return self.rows[0][0] +
               self.rows[1][1] +
               self.rows[2][2]
    } 

    pub fn determinant(&self) -> f64 {
        let i = self.rows[0][0] * (self.rows[1][1] * self.rows[2][2] -
                                   self.rows[1][2] * self.rows[2][1]);
        
        let j = self.rows[0][1] * (self.rows[1][0] * self.rows[2][2] -
                                   self.rows[1][2] * self.rows[2][0]);
        
        let k = self.rows[0][2] * (self.rows[1][0] * self.rows[2][1] -
                                   self.rows[1][1] * self.rows[2][0]);

        return i - j + k
    }

    pub fn transpose(&self) -> Mat3{
        let transpose: Mat3;
        for i in 0..2 {
            for j in 0..2 {
                transpose.rows[i][j] = self.rows[j][i];
            }
        }
        return transpose
    } 


    pub fn inverse() -> Mat3{
        
    }

    pub fn minor(i: i64, j: i64) -> Mat2{
        
    }

    pub fn cofactor(i: i64, j: i64) -> f64{
        
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

impl std::ops::AddAssign<f64> for Mat3 {
    fn add_assign(&mut self, scalar: f64) {
        self.rows[0] += scalar;
        self.rows[1] += scalar;
    }
}

impl std::ops::AddAssign<Mat3> for Mat3 {
    fn add_assign(&mut self, other: Mat3) {
        self.rows[0] += other.rows[0];
        self.rows[1] += other.rows[1];
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

impl std::ops::MulAssign<f64> for Mat3 {
    fn mul_assign(&mut self, scalar: f64) {
        self.rows[0] *= scalar;
        self.rows[1] *= scalar;
    }
}

impl std::ops::MulAssign<Mat3> for Mat3 {
    fn mul_assign(&mut self, other: Mat3) {
        self.rows[0] *= other.rows[0];
        self.rows[1] *= other.rows[1];
    }
}