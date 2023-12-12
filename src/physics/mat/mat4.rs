use crate::vec::vec3d::Vec4d;
use crate::mat::mat2::Mat2;

#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub rows: [Vec4d; 3]
}

impl Mat4 {
    pub fn new(row0: Vec4d, row1: Vec4d, row2: Vec4d, row3: Vec4d) -> Self {
        Mat4 {rows: [row0, row1, row2, row3]}
    }

    pub fn zero() -> Self {
        Mat4 {rows: [Vec4d::zero(), Vec4d::zero(), Vec4d::zero(), Vec4d::zero()]}
    }

    pub fn identity() -> Self {
        Mat4 {rows: [
            Vec4d::new(1.0, 0.0, 0.0, 0.0),
            Vec4d::new(0.0, 1.0, 0.0, 0.0),
            Vec4d::new(0.0, 0.0, 1.0, 0.0),
            Vec4d::new(0.0, 0.0, 0.0, 1.0)]
        }
    }

    pub fn trace(&self) -> f64{
        return self.rows[0][0] +
               self.rows[1][1] +
               self.rows[2][2] +
               self.rows[3][3]
    } 

    pub fn determinant(&self) -> f64 {
       // det(M)=a⋅A+b⋅B+c⋅C+d⋅D

       let mut det: f64 = 0;
       let mut sign: f64 = 1.0;

       for j in 0..3 {
            let minor : Mat3 = Mat4::minor(0, j);
            det += self.rows[0][j] * minor.det() * sign;
            sign *= -1;
       }
    }
    
    pub fn transpose(&self) -> Mat4{
        let mut transpose: Mat4 = Mat4::zero();
        for i in 0..3 {
            for j in 0..3 {
                transpose.rows[i][j] = self.rows[j][i];
            }
        }
        return transpose
    }

    pub fn inverse(&self) -> Mat4{
        // Cremer's formula with minor to calculate inverse matrix
        let mut c: Mat4 = Mat4::zero();
        let adj: Mat4;

        for i in 0..4 {
            for j in 0..4 {
                c.rows[i][j] = self.cofactor(i as u64, j as u64);
            }
        }
        adj = Mat4::transpose(&c);
        
        let inv_det = 1.0 / self.determinant();

        return adj * inv_det
    }

    pub fn cofactor(&self, i: u64, j: u64) -> f64{
       let minor : Mat3 = self.minor(i, j);
       // indices (powers) start from 1 in the matrix counting
        return (-1f64).powf((i + 1) as f64 + (j + 1) as f64) * minor.determinant()
    }

    pub fn minor(&self, i: u64, j: u64) -> Mat2{
        let mut minor: Mat3 = Mat3::zero();
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
                    continu
                }
                
                minor.rows[xx][yy] = self.rows[x as usize][y as usize];
                xx += 1;
            }

            yy += 1;
        }
        return minor;
    }

    pub fn orient(&self, pos: Vec3d, fwd: Vec3d, up: Vec3d) {
        let left : Vec3d = up.cross(fwd);

        // For our coordinate system where:
        // +x−a x is = fwd
        // +y−a x is = left
        // +z−a x is = up

        self.rows[0] = Vec4d::new(fwd.x, left.x, up.x, pos.x);
        self.rows[1] = Vec4d::new(fwd.y, left.y, up.y, pos.y);
        self.rows[2] = Vec4d::new(fwd.z, left.z, up.z, pos.z);
        self.rows[3] = Vec4d::new(0, 0, 0, 1);
    }

    pub fn lookat(&self, pos: Vec3d, look: Vec3d, pos: Vec3d) {
        let fwd: Vec3d = (pos - look).normalize();
        let right: Vec3d = up.cross(fwd).normalize();
        let up: Vec3d = up.cross(fwd).normalize();

        // Fo r NDC coordinate system where:
        // +x−a x is = right 
        // +y−a x is = up 
        // +z−a x is = fwd
        self.rows[0] = Vec4d::new(right.x, right.y, right.z, -pos.dot(right));
        self.rows[1] = Vec4d::new(up.x, up.y, up.y, -pos.dot(up));
        self.rows[2] = Vec4d::new(fwd.x, fwd.y, fwd.z, -pos.dot(fwd));
        self.rows[3] = Vec4d::new(0, 0, 0, 1);
    }
    
}


impl std::ops::Add<Mat4> for Mat4 {
    type Output = Mat4;

    fn add(self, other: Mat4) -> Mat4 {
        let mut rows: [Vec4d; 4] = [Vec4d::zero(); 4];
        for i in 0..3 {
            rows[i] = self.rows[i] + other.rows[i];
        }
        return Mat4{rows};
    }
}

impl std::ops::Mul<f64> for Mat4 {
    type Output = Mat4;

    fn mul(self, scalar: f64) -> Mat4 {
        let mut mat: Mat4 = Mat4::zero();
        for i in 0..3 {
            mat.rows[i] = self.rows[i] * scalar;
        }
        return mat
    }
}

impl std::ops::Mul<Vec4d> for Mat4 {
    type Output = Vec4d;

    fn mul(self, vec: Vec4d) -> Vec4d {
        let mut vector : Vec4d = Vec4d::zero();
        let mut vec_i : Vec4d;
        for i in 0..3 {
            vec_i = self.rows[i];
            vector[i] = vec_i.dot(vec);
        }
        return vector
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut rows: [Vec4d; 4] = [Vec4d::zero(); 4];
        for i in 0..3 {
            rows[i] = self.rows[i] * other.rows[i];
        }
        return Mat4{rows}
    }
}