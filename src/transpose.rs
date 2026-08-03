use crate::complex::Complex;
use crate::matrix::Matrix;
use std::ops::Neg;

impl<K: Copy + Default> Matrix<K> {
    pub fn transpose(&self) -> Matrix<K> {
        let mut new = vec![K::default(); self.width() * self.height()];
        for i in 0..self.height() {
            for j in 0..self.width() {
                new[j * self.height() + i] = *self.take(i, j);
            }
        }
        Matrix::from_buff(new, self.height(), self.width()).unwrap()
    }
}

impl<K: Copy + Default + Neg<Output = K>> Matrix<Complex<K>> {
    pub fn dagger(&self) -> Matrix<Complex<K>> {
        let mut new = vec![Complex::<K>::default(); self.width() * self.height()];
        for i in 0..self.height() {
            for j in 0..self.width() {
                new[j * self.height() + i] = self.take(i, j).conj();
            }
        }
        Matrix::from_buff(new, self.height(), self.width()).unwrap()
    }
}
