use std::ops::Add;

use num_traits::Zero;

use crate::matrix::Matrix;

impl<K: Copy + Add<Output = K> + Zero> Matrix<K> {
    pub fn try_trace(&self) -> Result<K, &str> {
        match self.width() == self.height() {
            true => Ok(self
                .get_buff_ref()
                .iter()
                .step_by(self.width() + 1)
                .fold(K::zero(), |acc, x| acc + *x)),
            false => Err("Matrix is not square"),
        }
    }

    pub fn trace(&self) -> K {
        self.try_trace().unwrap()
    }
}
