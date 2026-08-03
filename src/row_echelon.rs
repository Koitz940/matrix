use std::cmp::min;
use std::ops::{Div, DivAssign, SubAssign};

use num_traits::{Num, Zero};

use crate::matrix::Matrix;
use crate::vector::Vector;

impl<K> Matrix<K>
where
    K: Copy + Zero + Div<Output = K> + PartialEq + DivAssign + Num + SubAssign,
{
    pub fn rr(&self) -> Matrix<K> {
        let mut copy: Vec<Vector<K>> = (0..self.height())
            .map(|i| {
                let row = (0..self.width()).map(|j| *self.take(i, j)).collect();
                Vector::from(row)
            })
            .collect();
        let iterations = min(self.height(), self.width());
        for i in 0..iterations {
            let mut flag = false;
            for j in i..self.height() {
                if copy[j][i] != K::zero() {
                    copy.swap(i, j);
                    flag = true;
                    break;
                }
            }
            if !flag {
                continue;
            }
            let k = copy[i][i];
            copy[i] /= k;
            for j in (i + 1)..self.height() {
                let updated = &copy[i] * copy[j][i];
                copy[j].sub(&updated);
            }
        }
        Matrix::from_multiple(copy.into_iter().map(|row| row.get_buff()).collect()).unwrap()
    }

    pub fn stand_row_echelon(&self) -> Matrix<K> {
        let mut copy: Vec<Vector<K>> = (0..self.height())
            .map(|i| {
                let row = (0..self.width()).map(|j| *self.take(i, j)).collect();
                Vector::from(row)
            })
            .collect();

        let mut pivot_row = 0;

        for col in 0..self.width() {
            if pivot_row >= self.height() {
                break;
            }

            let mut flag = false;
            for j in pivot_row..self.height() {
                if copy[j][col] != K::zero() {
                    copy.swap(pivot_row, j);
                    flag = true;
                    break;
                }
            }

            if !flag {
                continue;
            }

            let k = copy[pivot_row][col];

            for j in (pivot_row + 1)..self.height() {
                let factor = copy[j][col];
                if factor != K::zero() {
                    let updated = &copy[pivot_row] * (factor / k);
                    copy[j].sub(&updated);
                }
            }

            pivot_row += 1;
        }

        Matrix::from_multiple(copy.into_iter().map(|row| row.get_buff()).collect()).unwrap()
    }

    pub fn reduced_row_echelon(&self) -> Matrix<K> {
        let mut copy: Vec<Vector<K>> = (0..self.height())
            .map(|i| {
                let row = (0..self.width()).map(|j| *self.take(i, j)).collect();
                Vector::from(row)
            })
            .collect();

        let mut pivot_row = 0;

        for col in 0..self.width() {
            if pivot_row >= self.height() {
                break;
            }

            let mut swap_row = None;
            for r in pivot_row..self.height() {
                if copy[r][col] != K::zero() {
                    swap_row = Some(r);
                    break;
                }
            }

            let Some(r) = swap_row else {
                continue;
            };

            copy.swap(pivot_row, r);

            let pivot_val = copy[pivot_row][col];
            copy[pivot_row] /= pivot_val;

            for r in 0..self.height() {
                if r != pivot_row {
                    let factor = copy[r][col];
                    if factor != K::zero() {
                        let updated = &copy[pivot_row] * factor;
                        copy[r] -= updated;
                    }
                }
            }

            pivot_row += 1;
        }

        Matrix::from_multiple(copy.into_iter().map(|row| row.get_buff()).collect()).unwrap()
    }

    pub fn row_echelon(&self) -> Matrix<K> {
        self.reduced_row_echelon()
    }
}
