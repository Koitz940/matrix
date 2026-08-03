use crate::matrix::Matrix;
use crate::vector::Vector;
use std::{
    cmp::min,
    ops::{Div, DivAssign, MulAssign, Neg, SubAssign},
};

use num_traits::{Num, One, Zero};

impl<K> Matrix<K>
where
    K: Copy
        + One
        + Zero
        + Div<Output = K>
        + PartialEq
        + DivAssign
        + Num
        + SubAssign
        + MulAssign
        + Neg<Output = K>,
{
    pub fn try_det(&self) -> Result<K, &str> {
        if self.width() != self.height() {
            Err("Attempted to calculate determinant of a non square matrix")
        } else {
            let mut copy: Vec<Vector<K>> = (0..self.height())
                .map(|i| {
                    let row = (0..self.width()).map(|j| *self.take(i, j)).collect();
                    Vector::from(row)
                })
                .collect();
            let mut det = K::one();
            let mut sign = K::one();
            let iterations = min(self.height(), self.width());
            for i in 0..iterations {
                let mut flag = false;
                for j in i..self.height() {
                    if copy[j][i] != K::zero() {
                        copy.swap(i, j);
                        if j != i {
                            sign = -sign;
                        }
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    return Ok(K::zero());
                }
                let k = copy[i][i];
                det *= k;
                for j in (i + 1)..self.height() {
                    let updated = &copy[i] * (copy[j][i] / k);
                    copy[j].sub(&updated);
                }
            }
            Ok(det * sign)
        }
    }
    pub fn determinant(&self) -> K {
        self.try_det().unwrap()
    }
}
