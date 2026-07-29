use std::ops::{Add, Mul, Sub};

use num_traits::One;

pub fn lerp<V, W, T>(u: V, v: W, t: T) -> V
where
    V: Add<W, Output = V> + Mul<T, Output = V>,
    W: Mul<T, Output = W>,
    T: Copy + One + Sub<Output = T>,
{
    u * t + v * (T::one() - t)
}
