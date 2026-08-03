use num_traits::Num;

use crate::matrix::Matrix;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<K: Copy + Add<Output = K>> Matrix<K> {
    pub fn try_sum(&self, other: &Matrix<K>) -> Result<Matrix<K>, &str> {
        match self.dim() == other.dim() {
            true => {
                let new = self
                    .iter()
                    .zip(other)
                    .map(|(x, y): (&K, &K)| *x + *y)
                    .collect();
                Ok(Matrix::from_buff(new, self.width(), self.height()).unwrap())
            }
            false => Err("Vectro Dimensions don't match"),
        }
    }
}

impl<K: Copy + Sub<Output = K>> Matrix<K> {
    pub fn try_sub(&self, other: &Matrix<K>) -> Result<Matrix<K>, &str> {
        match self.dim() == other.dim() {
            true => {
                let new = self
                    .iter()
                    .zip(other)
                    .map(|(x, y): (&K, &K)| *x - *y)
                    .collect();
                Ok(Matrix::from_buff(new, self.width(), self.height()).unwrap())
            }
            false => Err("Vectro Dimensions don't match"),
        }
    }
}

impl<K: Copy + Add<Output = K>> Add<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: &Matrix<K>) -> Matrix<K> {
        self.try_sum(other).unwrap()
    }
}

impl<K: Copy + Add<Output = K>> Add<Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: Matrix<K>) -> Matrix<K> {
        self.try_sum(&other).unwrap()
    }
}

impl<K: Copy + Add<Output = K>> Add<Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: Matrix<K>) -> Matrix<K> {
        self.try_sum(&other).unwrap()
    }
}

impl<K: Copy + Add<Output = K>> Add<&Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;
    fn add(self, other: &Matrix<K>) -> Matrix<K> {
        self.try_sum(other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<&Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, other: &Matrix<K>) -> Matrix<K> {
        self.try_sub(other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<Matrix<K>> for &Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        self.try_sub(&other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        self.try_sub(&other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<&Matrix<K>> for Matrix<K> {
    type Output = Matrix<K>;
    fn sub(self, other: &Matrix<K>) -> Matrix<K> {
        self.try_sub(other).unwrap()
    }
}

impl<R: Copy + Num, K: Copy + Mul<R, Output = K>> Mul<R> for &Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, other: R) -> Matrix<K> {
        let new = self.iter().map(|x| *x * other).collect();

        Matrix::from_buff(new, self.width(), self.height()).unwrap()
    }
}

impl<R: Copy + Num, K: Copy + Mul<R, Output = K>> Mul<R> for Matrix<K> {
    type Output = Matrix<K>;
    fn mul(self, other: R) -> Matrix<K> {
        let new = self.iter().map(|x| *x * other).collect();

        Matrix::from_buff(new, self.width(), self.height()).unwrap()
    }
}

impl<K: Copy + SubAssign> SubAssign for Matrix<K> {
    fn sub_assign(&mut self, other: Self) {
        Matrix::sub(self, &other);
    }
}

impl<K: Copy + AddAssign> AddAssign for Matrix<K> {
    fn add_assign(&mut self, other: Self) {
        Matrix::add(self, &other);
    }
}

impl<K: Copy + MulAssign> MulAssign<K> for Matrix<K> {
    fn mul_assign(&mut self, other: K) {
        Matrix::scl(self, other);
    }
}

impl<K: Copy + SubAssign> SubAssign<&Matrix<K>> for Matrix<K> {
    fn sub_assign(&mut self, other: &Self) {
        Matrix::sub(self, &other);
    }
}

impl<K: Copy + AddAssign> AddAssign<&Matrix<K>> for Matrix<K> {
    fn add_assign(&mut self, other: &Self) {
        Matrix::add(self, &other);
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Matrix<T> {
        Matrix::from_buff(
            self.get_buff_ref().iter().map(|a| -*a).collect(),
            self.width(),
            self.height(),
        )
        .unwrap()
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Matrix<T> {
    type Output = Matrix<T>;
    fn neg(self) -> Matrix<T> {
        Matrix::from_buff(
            self.get_buff_ref().iter().map(|a| -*a).collect(),
            self.width(),
            self.height(),
        )
        .unwrap()
    }
}
