use crate::complex::RealMagnitude;
use num_traits::Float;
use num_traits::Zero;
use std::ops::{Add, Mul};

use crate::vector::Vector;

impl<R, K> Vector<K>
where
    R: Add<Output = R> + Zero,
    K: Copy + RealMagnitude<Real = R>,
{
    pub fn cool_norm_1(&self) -> R {
        self.iter().fold(R::zero(), |acc, x| acc + x.abs_val())
    }
}

impl<R, K> Vector<K>
where
    R: Add<Output = R> + Zero + Mul<Output = R> + Float,
    K: RealMagnitude<Real = R>,
{
    pub fn cool_norm(&self) -> R {
        self.iter()
            .fold(R::zero(), |acc, x| acc + x.abs_val() * x.abs_val())
            .sqrt()
    }
}

impl<R, K> Vector<K>
where
    R: PartialOrd + Default,
    K: RealMagnitude<Real = R>,
{
    pub fn cool_norm_inf(&self) -> R {
        if self.dim() == 0 {
            return R::default();
        }
        self.iter().fold(self[0].abs_val(), |acc, x| {
            if acc > x.abs_val() { acc } else { x.abs_val() }
        })
    }
}

impl<K> Vector<K>
where
    K: Copy + RealMagnitude<Real = f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.iter().fold(f32::zero(), |acc, x| acc + x.abs_val())
    }
}

impl<K> Vector<K>
where
    K: RealMagnitude<Real = f32>,
{
    pub fn norm(&self) -> f32 {
        self.iter()
            .fold(f32::zero(), |acc, x| acc + x.abs_val() * x.abs_val())
            .powf(0.5)
    }
}

impl<K> Vector<K>
where
    K: RealMagnitude<Real = f32>,
{
    pub fn norm_inf(&self) -> f32 {
        if self.dim() == 0 {
            return f32::default();
        }
        self.iter().fold(self[0].abs_val(), |acc, x| {
            if acc > x.abs_val() { acc } else { x.abs_val() }
        })
    }
}

impl<K> Vector<K>
where
    K: Copy + RealMagnitude<Real = f64>,
{
    pub fn f64_norm_1(&self) -> f64 {
        self.iter().fold(f64::zero(), |acc, x| acc + x.abs_val())
    }
}

impl<K> Vector<K>
where
    K: RealMagnitude<Real = f64>,
{
    pub fn f64_norm(&self) -> f64 {
        self.iter()
            .fold(f64::zero(), |acc, x| acc + x.abs_val() * x.abs_val())
            .powf(0.5)
    }
}

impl<K> Vector<K>
where
    K: RealMagnitude<Real = f64>,
{
    pub fn f64_norm_inf(&self) -> f64 {
        if self.dim() == 0 {
            return f64::default();
        }
        self.iter().fold(self[0].abs_val(), |acc, x| {
            if acc > x.abs_val() { acc } else { x.abs_val() }
        })
    }
}
