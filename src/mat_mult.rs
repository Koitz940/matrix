use std::ops::Mul;

use num_traits::{MulAdd, Zero};

use crate::matrix::Matrix;
use crate::vector::Vector;

impl<K: Copy + MulAdd<Output = K> + Zero> Matrix<K> {
    pub fn ref_mul_vec(&self, vec: &Vector<K>) -> Result<Vector<K>, &str> {
        if vec.dim() != self.width() {
            Err("Mismatched matrix and vector Dimensions")
        } else {
            let mut new = vec![K::zero(); self.height()];
            for i in 0..self.height() {
                for j in 0..self.height() {
                    new[i] = self.take(i, j).mul_add(vec[j], new[i]);
                }
            }
            Ok(Vector::from(new))
        }
    }

    pub fn ref_mul_mat(&self, mat: &Matrix<K>) -> Result<Matrix<K>, &str> {
        if mat.height() != self.width() {
            Err("Mismatched matricies Dimensions")
        } else {
            let mut new = vec![K::zero(); self.height() * mat.width()];
            for i in 0..self.height() {
                for j in 0..mat.width() {
                    for k in 0..self.width() {
                        new[i * mat.width() + j] = self
                            .take(i, k)
                            .mul_add(*mat.take(k, j), new[i * mat.width() + j])
                    }
                }
            }
            Ok(Matrix::from_buff(new, self.height(), mat.width()).unwrap())
        }
    }

    pub fn ref_mul_vec_mat(&self, vec: &Vector<K>) -> Result<Vector<K>, &str> {
        if vec.dim() != self.height() {
            Err("Mismatched matrix and vector Dimensions")
        } else {
            let mut new = vec![K::zero(); self.width()];
            for j in 0..self.width() {
                for i in 0..self.height() {
                    new[i] = self.take(i, j).mul_add(vec[j], new[i]);
                }
            }
            Ok(Vector::from(new))
        }
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Matrix<K> {
    pub fn mul_vec(&self, vec: Vector<K>) -> Vector<K> {
        self.ref_mul_vec(&vec).unwrap()
    }

    pub fn mul_mat(&self, mat: Matrix<K>) -> Matrix<K> {
        self.ref_mul_mat(&mat).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, mat: Matrix<K>) -> Self::Output {
        self.ref_mul_mat(&mat).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, mat: Matrix<K>) -> Self::Output {
        self.ref_mul_mat(&mat).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, mat: &Matrix<K>) -> Self::Output {
        self.ref_mul_mat(mat).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Matrix<K>> for Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: Matrix<K>) -> Self::Output {
        rhs.ref_mul_vec_mat(&self).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Matrix<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: Matrix<K>) -> Self::Output {
        rhs.ref_mul_vec_mat(self).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<&Matrix<K>> for Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: &Matrix<K>) -> Self::Output {
        rhs.ref_mul_vec_mat(&self).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<&Matrix<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: &Matrix<K>) -> Self::Output {
        rhs.ref_mul_vec_mat(self).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<&Vector<K>> for &Matrix<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        self.ref_mul_vec(rhs).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Vector<K>> for &Matrix<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: Vector<K>) -> Self::Output {
        self.ref_mul_vec(&rhs).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<&Vector<K>> for Matrix<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: &Vector<K>) -> Self::Output {
        self.ref_mul_vec(rhs).unwrap()
    }
}

impl<K: Copy + MulAdd<Output = K> + Zero> Mul<Vector<K>> for Matrix<K> {
    type Output = Vector<K>;
    fn mul(self, rhs: Vector<K>) -> Self::Output {
        self.ref_mul_vec(&rhs).unwrap()
    }
}
