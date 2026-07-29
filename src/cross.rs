use std::ops::{Mul, Sub};

use crate::vector::Vector;

pub fn cross_product<K: Sub<Output = K> + Mul<Output = K> + Copy>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> Vector<K> {
    assert!(
        u.dim() == 3 && v.dim() == 3,
        "Bad Dimensions on cross product input"
    );
    Vector::from(vec![
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - v[0] * u[1],
    ])
}
