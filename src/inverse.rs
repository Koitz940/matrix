use crate::matrix::Matrix;
use crate::vector::Vector;
use std::{
    ops::{Div, DivAssign, SubAssign},
};

use num_traits::{Num, One, Zero};

impl<K> Matrix<K>
where
    K: Copy + Zero + One + Div<Output = K> + PartialEq + DivAssign + Num + SubAssign,
{
    pub fn inverse(&self) -> Result<Matrix<K>, &str> {
        if self.width() != self.height() {
            return Err("Attempted to take inverse of a non square matrix");
        }

        let mut copy: Vec<Vector<K>> = (0..self.height())
            .map(|i| {
                let row = (0..self.width()).map(|j| *self.take(i, j)).collect();
                Vector::from(row)
            })
            .collect();

        let mut inv: Vec<Vector<K>> = (0..self.height())
            .map(|i| {
                let row = (0..self.width())
                    .map(|j| if i == j { K::one() } else { K::zero() })
                    .collect();
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
                return Err("Attempted to take inverse of singular matrix");
            };

            copy.swap(pivot_row, r);
			inv.swap(pivot_row, r);

            let pivot_val = copy[pivot_row][col];
            copy[pivot_row] /= pivot_val;
			inv[pivot_row] /= pivot_val;

            for r in 0..self.height() {
                if r != pivot_row {
                    let factor = copy[r][col];
                    if factor != K::zero() {
                        let updated = &copy[pivot_row] * factor;
                        copy[r] -= updated;
						let updated = &inv[pivot_row] * factor;
                        inv[r] -= updated;
                    }
                }
            }

            pivot_row += 1;
        }

        Ok(Matrix::from_multiple(inv.into_iter().map(|row| row.get_buff()).collect()).unwrap())
    }
}
