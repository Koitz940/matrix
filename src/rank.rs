use std::ops::{Div, DivAssign, SubAssign};

use num_traits::{Num, Zero};

use crate::matrix::Matrix;

impl<K> Matrix<K>
where
    K: Copy + Zero + Div<Output = K> + PartialEq + DivAssign + Num + SubAssign + PartialEq,
{
	pub fn rank(&self) -> usize {
		let r = self.row_echelon();
		let mut n = 0;
		'outer: for i in 0..self.height() {
			for j in 0..self.width() {
				if *r.take(i, j) != K::zero() {
					n += 1;
					continue 'outer;
				}
			}
			break;
		}
		n
	}
}