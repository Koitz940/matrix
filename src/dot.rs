use crate::complex::Complex;
use crate::vector::Vector;
use num_traits::Zero;
use num_traits::ops::mul_add::MulAdd;
use std::ops::{Mul, Neg, Sub};

impl<K: MulAdd<K, K, Output = K> + Copy + Zero> Vector<K> {
    pub fn dot(&self, other: &Vector<K>) -> K {
        self.iter()
            .zip(other)
            .fold(K::zero(), |acc, (x, y)| x.mul_add(*y, acc))
    }
}

impl<
    K: Sub<Output = K> + MulAdd<K, K, Output = K> + Copy + Zero + Mul<Output = K> + Neg<Output = K> + PartialEq,
> Vector<Complex<K>>
{
    pub fn dot_conj(&self, other: &Vector<Complex<K>>) -> Complex<K> {
        self.iter()
            .zip(other)
            .fold(Complex::<K>::zero(), |acc, (x, y)| x.mul_add(y.conj(), acc))
    }
}
