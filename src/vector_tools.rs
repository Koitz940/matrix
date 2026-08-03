use num_traits::Num;

use crate::vector::Vector;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl<K: Copy + Add<Output = K>> Vector<K> {
    pub fn try_sum(&self, other: &Vector<K>) -> Result<Vector<K>, &str> {
        match self.dim() == other.dim() {
            true => Ok(Vector::from(
                self.iter()
                    .zip(other)
                    .map(|(x, y): (&K, &K)| *x + *y)
                    .collect(),
            )),
            false => Err("Vectro Dimensions don't match"),
        }
    }
}

impl<K: Copy + Sub<Output = K>> Vector<K> {
    pub fn try_sub(&self, other: &Vector<K>) -> Result<Vector<K>, &str> {
        match self.dim() == other.dim() {
            true => Ok(Vector::from(
                self.iter()
                    .zip(other)
                    .map(|(x, y): (&K, &K)| *x - *y)
                    .collect(),
            )),
            false => Err("Vectro Dimensions don't match"),
        }
    }
}

impl<K: Copy + Add<Output = K>> Add<&Vector<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn add(self, other: &Vector<K>) -> Vector<K> {
        self.try_sum(other).unwrap()
    }
}

impl<K: Copy + Add<Output = K>> Add<Vector<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn add(self, other: Vector<K>) -> Vector<K> {
        self.try_sum(&other).unwrap()
    }
}

impl<K: Copy + Add<Output = K>> Add<&Vector<K>> for Vector<K> {
    type Output = Vector<K>;

    fn add(self, other: &Vector<K>) -> Vector<K> {
        &self + other
    }
}

impl<K: Copy + Add<Output = K>> Add<Vector<K>> for Vector<K> {
    type Output = Vector<K>;

    fn add(self, other: Vector<K>) -> Vector<K> {
        &self + other
    }
}

impl<K: Copy + Sub<Output = K>> Sub<&Vector<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn sub(self, other: &Vector<K>) -> Vector<K> {
        self.try_sub(other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<Vector<K>> for &Vector<K> {
    type Output = Vector<K>;
    fn sub(self, other: Vector<K>) -> Vector<K> {
        self.try_sub(&other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<&Vector<K>> for Vector<K> {
    type Output = Vector<K>;
    fn sub(self, other: &Vector<K>) -> Vector<K> {
        self.try_sub(&other).unwrap()
    }
}

impl<K: Copy + Sub<Output = K>> Sub<Vector<K>> for Vector<K> {
    type Output = Vector<K>;
    fn sub(self, other: Vector<K>) -> Vector<K> {
        self.try_sub(&other).unwrap()
    }
}

impl<R: Copy + Num, K: Copy + Mul<R, Output = K>> Mul<R> for &Vector<K> {
    type Output = Vector<K>;
    fn mul(self, other: R) -> Vector<K> {
        let new = self.iter().map(|x| *x * other).collect();

        Vector::from(new)
    }
}

impl<R: Copy + Num, K: Copy + Mul<R, Output = K>> Mul<R> for Vector<K> {
    type Output = Vector<K>;
    fn mul(self, other: R) -> Vector<K> {
        let new = self.iter().map(|x| *x * other).collect();

        Vector::from(new)
    }
}

impl<R: Copy + Num, K: Copy + Div<R, Output = K>> Div<R> for &Vector<K> {
    type Output = Vector<K>;
    fn div(self, other: R) -> Vector<K> {
        let new = self.iter().map(|x| *x / other).collect();

        Vector::from(new)
    }
}

impl<R: Copy + Num, K: Copy + Div<R, Output = K>> Div<R> for Vector<K> {
    type Output = Vector<K>;
    fn div(self, other: R) -> Vector<K> {
        let new = self.iter().map(|x| *x / other).collect();

        Vector::from(new)
    }
}

impl<K: Copy + SubAssign> SubAssign for Vector<K> {
    fn sub_assign(&mut self, other: Self) {
        Vector::sub(self, &other);
    }
}

impl<K: Copy + AddAssign> AddAssign for Vector<K> {
    fn add_assign(&mut self, other: Self) {
        Vector::add(self, &other);
    }
}

impl<K: Copy + MulAssign> MulAssign<K> for Vector<K> {
    fn mul_assign(&mut self, other: K) {
        Vector::scl(self, other);
    }
}

impl<K: Copy + DivAssign> DivAssign<K> for Vector<K> {
    fn div_assign(&mut self, other: K) {
        Vector::dcl(self, other);
    }
}

impl<K: Copy + SubAssign> SubAssign<&Vector<K>> for Vector<K> {
    fn sub_assign(&mut self, other: &Self) {
        Vector::sub(self, &other);
    }
}

impl<K: Copy + AddAssign> AddAssign<&Vector<K>> for Vector<K> {
    fn add_assign(&mut self, other: &Self) {
        Vector::add(self, &other);
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Vector<T> {
        self.get_buff_ref().iter().map(|a| -*a).collect()
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vector<T> {
    type Output = Vector<T>;
    fn neg(self) -> Vector<T> {
        self.get_buff_ref().iter().map(|a| -*a).collect()
    }
}
