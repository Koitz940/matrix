use std::iter::repeat_n;
use std::ops::{Add, Mul};

use num_traits::Num;

use crate::vector::Vector;

pub fn linear_combination<K, T: Num>(u: &[Vector<K>], coefs: &[T]) -> Vector<K>
where
    K: Default + Add<Output = K> + Mul<T, Output = K> + Copy,
    T: Copy,
{
    assert_eq!(
        u.len(),
        coefs.len(),
        "Bad linear combination, there are {} vectors but {} coefficients",
        u.len(),
        coefs.len()
    );
    if u.is_empty() {
        return Vector::from(vec![]);
    }
    let init = Vector::from(repeat_n(K::default(), u.first().unwrap().dim()).collect());
    u.iter().zip(coefs).fold(init, |acc, (v, &c)| acc + (v * c))
}
