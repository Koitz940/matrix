use crate::matrix::Matrix;
use crate::vector::Vector;
use std::ops::{AddAssign, MulAssign, SubAssign};

impl<K: AddAssign + Copy> Vector<K> {
    pub fn try_add_assign(&mut self, v: &Vector<K>) -> Result<(), &str> {
        match self.dim() == v.dim() {
            true => Ok(self
                .get_mut_buff_ref()
                .iter_mut()
                .zip(v.get_buff_ref())
                .for_each(|(x, y)| *x += *y)),
            false => Err("Vectro Dimensions don't match"),
        }
    }

    pub fn add(&mut self, v: &Vector<K>) {
        self.try_add_assign(v).unwrap();
    }
}

impl<K: SubAssign + Copy> Vector<K> {
    pub fn try_sub_assign(&mut self, v: &Vector<K>) -> Result<(), &str> {
        match self.dim() == v.dim() {
            true => Ok(self
                .get_mut_buff_ref()
                .iter_mut()
                .zip(v.get_buff_ref())
                .for_each(|(x, y)| *x -= *y)),
            false => Err("Vectro Dimensions don't match"),
        }
    }

    pub fn sub(&mut self, v: &Vector<K>) {
        self.try_sub_assign(v).unwrap();
    }
}

impl<K> Vector<K> {
    pub fn scl<T: Copy>(&mut self, a: T)
    where
        K: MulAssign<T>,
    {
        self.get_mut_buff_ref().iter_mut().for_each(|x| *x *= a)
    }
}

impl<K: AddAssign + Copy> Matrix<K> {
    pub fn try_add_assign(&mut self, v: &Matrix<K>) -> Result<(), &str> {
        if self.width() == v.width() && self.height() == v.height() {
            Ok(self
                .get_buff_ref_mut()
                .iter_mut()
                .zip(v.get_buff_ref())
                .for_each(|(x, y)| *x += *y))
        } else {
            Err("Attempted to add matricies of distintc Dimensions")
        }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        self.try_add_assign(v).unwrap()
    }
}

impl<K: SubAssign + Copy> Matrix<K> {
    pub fn try_sub_assign(&mut self, v: &Matrix<K>) -> Result<(), &str> {
        if self.width() == v.width() && self.height() == v.height() {
            Ok(self
                .get_buff_ref_mut()
                .iter_mut()
                .zip(v.get_buff_ref())
                .for_each(|(x, y)| *x -= *y))
        } else {
            Err("Attempted to add matricies of distintc Dimensions")
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        self.try_sub_assign(v).unwrap()
    }
}

impl<K: MulAssign + Copy> Matrix<K> {
    pub fn scl(&mut self, a: K) {
        self.get_buff_ref_mut().iter_mut().for_each(|x| *x *= a)
    }
}
