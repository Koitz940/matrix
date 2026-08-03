use num_traits::{Float, MulAdd, Signed, Zero};

use crate::complex::{Complex, RealMagnitude};
use crate::vector::Vector;

/* pub fn cool_angle_cos<K: MulAdd<K, K, Output = K> + Copy + Zero + Float + Signed>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> K {
    Vector::dot(u, v) / (u.cool_norm() * v.cool_norm())
}

pub fn cool_comp_angle_re_cos<K: MulAdd<K, K, Output = K> + Copy + Zero + Float + Signed>(
    u: &Vector<Complex<K>>,
    v: &Vector<Complex<K>>,
) -> K {
    Vector::dot_conj(u, v).re / (u.cool_norm() * v.cool_norm())
}

pub fn cool_comp_angle_abs_cos<K: MulAdd<K, K, Output = K> + Copy + Zero + Float + Signed>(
    u: &Vector<Complex<K>>,
    v: &Vector<Complex<K>>,
) -> K {
    Vector::dot_conj(u, v).modl().sqrt() / (u.cool_norm() * v.cool_norm())
} */

pub fn angle_cos<K: MulAdd<K, K, Output = K> + Copy + Zero + Float + Signed + RealMagnitude<Real = f32>>(
    u: &Vector<K>,
    v: &Vector<K>,
) -> f32 {
    Vector::dot(u, v).abs_val() / (u.norm() * v.norm())
}

pub fn cf32_angle_re_cos<>(
    u: &Vector<Complex<f32>>,
    v: &Vector<Complex<f32>>,
) -> f32 {
    Vector::dot_conj(u, v).re / (u.cool_norm() * v.cool_norm())
}

pub fn cf32_angle_abs_cos(
    u: &Vector<Complex<f32>>,
    v: &Vector<Complex<f32>>,
) -> f32 {
    Vector::dot_conj(u, v).abs_val() / (u.cool_norm() * v.cool_norm())
}

/* pub fn cf64_angle_re_cos<>(
    u: &Vector<Complex<f32>>,
    v: &Vector<Complex<f32>>,
) -> f32 {
    Vector::dot_conj(u, v).re / (u.cool_norm() * v.cool_norm())
}

pub fn cf64_angle_abs_cos(
    u: &Vector<Complex<f64>>,
    v: &Vector<Complex<f64>>,
) -> f64 {
    Vector::dot_conj(u, v).abs_val() / (u.cool_norm() * v.cool_norm())
}
 */