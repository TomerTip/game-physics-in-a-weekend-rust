use crate::vec::vec3d::Vec3d;
use crate::mat::mat2::Mat2;

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

    pub fn identity() -> Self {
        Mat3 {rows: [
            Vec3d::new(1.0, 0.0, 0.0),
            Vec3d::new(0.0, 1.0, 0.0),
            Vec3d::new(0.0, 0.0, 1.0)]
        }
    }

    pub fn trace(&self) -> f64{
        return self.rows[0][0] +
               self.rows[1][1] +
               self.rows[2][2]
    } 

    pub fn determinant(&self) -> f64 {
        let i : f64 = self.rows[0][0] * (self.rows[1][1] * self.rows[2][2] -
                                   self.rows[1][2] * self.rows[2][1]);
        
        let j = self.rows[0][1] * (self.rows[1][0] * self.rows[2][2] -
                                   self.rows[1][2] * self.rows[2][0]);
        
        let k = self.rows[0][2] * (self.rows[1][0] * self.rows[2][1] -
                                   self.rows[1][1] * self.rows[2][0]);

        return i - j + k
    }

    pub fn transpose(&self) -> Mat3{
        let mut transpose: Mat3 = Mat3::zero();
        for i in 0..2 {
            for j in 0..2 {
                transpose.rows[i][j] = self.rows[j][i];
            }
        }
        return transpose
    }

    pub fn inverse(&self) -> Mat3{
        // Cremer's formula with minor to calculate inverse matrix
        let mut c: Mat3 = Mat3::zero();
        let adj: Mat3;

        for i in 0..3 {
            for j in 0..3 {
                c.rows[i][j] = self.cofactor(i as u64, j as u64);
            }
        }
        adj = Mat3::transpose(&c);
        
        let inv_det = 1.0 / self.determinant();

        return adj * inv_det
    }

    pub fn cofactor(&self, i: u64, j: u64) -> f64{
       let minor = self.minor(i, j);
       // indices start from 1 in the matrix counting
        return (-1f64).powf((i + 1) as f64 + (j + 1) as f64) * minor.determinant()
    }

    pub fn minor(&self, i: u64, j: u64) -> Mat2{
        let mut minor: Mat2 = Mat2::zero();
        let mut xx: usize;
        let mut yy: usize;

        yy = 0;
        for y in 0..3 {
            if y == j {
                continue;
            }

            xx = 0;
            for x in 0..i {
                if x == i {
                    continue;
                }
                
                minor.rows[xx][yy] = self.rows[x as usize][y as usize];
                xx += 1;
            }

            yy += 1;
        }
        return minor;
    } 
    
}

impl std::ops::Add<Mat3> for Mat3 {
    type Output = Mat3;

    fn add(self, other: Mat3) -> Mat3 {
        let mut rows: [Vec3d; 3] = [Vec3d::zero(); 3];
        for i in 0..2 {
            rows[i] = self.rows[i] + other.rows[i];
        }
        return Mat3{rows};
    }
}

impl std::ops::Mul<f64> for Mat3 {
    type Output = Mat3;

    fn mul(self, scalar: f64) -> Mat3 {
        let mut mat: Mat3 = Mat3::zero();
        for i in 0..2 {
            mat.rows[i] = self.rows[i] * scalar;
        }
        return mat
    }
}

impl std::ops::Mul<Vec3d> for Mat3 {
    type Output = Vec3d;

    fn mul(self, vec: Vec3d) -> Vec3d {
        let mut vector : Vec3d = Vec3d::zero();
        let mut vec_i : Vec3d;
        for i in 0..2 {
            vec_i= self.rows[i];
            vector[i] = vec_i.dot(vec);
        }
        return vector
    }
}

impl std::ops::Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, other: Mat3) -> Mat3 {
        let mut rows: [Vec3d; 3] = [Vec3d::zero(); 3];
        for i in 0..2 {
            rows[i] = self.rows[i] * other.rows[i];
        }
        return Mat3{rows}
    }
}