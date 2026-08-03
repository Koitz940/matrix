use num_traits::{MulAdd, Num};
use num_traits::{One, Zero};
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Cyclic<const N: u32> {
    val: u32,
}

impl<const N: u32> Cyclic<N> {
    pub fn new(val: u32) -> Self {
        Cyclic { val: val % N }
    }

    pub fn new_i64(val: i64) -> Self {
        Cyclic {
            val: (val.rem_euclid(N as i64) as u32),
        }
    }

    pub fn get_val(&self) -> u32 {
        self.val
    }
}

impl<const N: u32> Add<Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Cyclic::<N>::new((self.val + rhs.val) % N)
    }
}

impl<const N: u32> Add<Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn add(self, rhs: Cyclic<N>) -> Self::Output {
        Cyclic::<N>::new((self.val + rhs.val) % N)
    }
}

impl<const N: u32> Add<&Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn add(self, rhs: &Self) -> Self::Output {
        Cyclic::new((self.val + rhs.val) % N)
    }
}

impl<const N: u32> Add<&Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn add(self, rhs: &Cyclic<N>) -> Self::Output {
        Cyclic::new((self.val + rhs.val) % N)
    }
}

impl<const N: u32> Sub<Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Cyclic::<N>::new((N + self.val - rhs.val) % N)
    }
}

impl<const N: u32> Sub<Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn sub(self, rhs: Cyclic<N>) -> Self::Output {
        Cyclic::<N>::new((N + self.val - rhs.val) % N)
    }
}

impl<const N: u32> Sub<&Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn sub(self, rhs: &Self) -> Self::Output {
        Cyclic::new((N + self.val - rhs.val) % N)
    }
}

impl<const N: u32> Sub<&Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn sub(self, rhs: &Cyclic<N>) -> Self::Output {
        Cyclic::new((N + self.val - rhs.val) % N)
    }
}

impl<const N: u32> Mul<Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn mul(self, rhs: Self) -> Self::Output {
        Cyclic::<N>::new((self.val * rhs.val) % N)
    }
}

impl<const N: u32> Mul<Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn mul(self, rhs: Cyclic<N>) -> Self::Output {
        Cyclic::<N>::new((self.val * rhs.val) % N)
    }
}

impl<const N: u32> Mul<&Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn mul(self, rhs: &Self) -> Self::Output {
        Cyclic::new((self.val * rhs.val) % N)
    }
}

impl<const N: u32> Mul<&Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn mul(self, rhs: &Cyclic<N>) -> Self::Output {
        Cyclic::new((self.val * rhs.val) % N)
    }
}

impl<const N: u32> One for Cyclic<N> {
    fn one() -> Self {
        Cyclic { val: 1 % N }
    }
}

impl<const N: u32> Zero for Cyclic<N> {
    fn zero() -> Self {
        Cyclic { val: 0 }
    }

    fn is_zero(&self) -> bool {
        return self.val == 0;
    }
}

impl<const N: u32> Cyclic<N> {
    pub fn inverse(&self) -> Result<Cyclic<N>, String> {
        let mut t: i64 = 0;
        let mut newt: i64 = 1;
        let mut r: i64 = N as i64;
        let mut newr = self.val as i64;

        if newr == 0 {
            return Err("Attempted to take the inverse of 0".to_string());
        }

        while newr != 0 {
            let c = r / newr;
            let temp = t;
            t = newt;
            newt = temp - c * newt;
            let temp = r;
            r = newr;
            newr = temp - c * newr;
        }

        if newr > 1 {
            return Err(self.val.to_string()
                + " is a divisior of "
                + &N.to_string()
                + " Therefore, it cannot get inverted");
        }
        if t < 0 {
            t += N as i64;
        }

        Ok(Cyclic { val: t as u32 })
    }
}

impl<const N: u32> Div<Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse().unwrap()
    }
}

impl<const N: u32> Div<&Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn div(self, rhs: &Self) -> Self::Output {
        self * rhs.inverse().unwrap()
    }
}

impl<const N: u32> Div<Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn div(self, rhs: Cyclic<N>) -> Self::Output {
        self * rhs.inverse().unwrap()
    }
}

impl<const N: u32> Div<&Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn div(self, rhs: &Cyclic<N>) -> Self::Output {
        self * rhs.inverse().unwrap()
    }
}

impl<const N: u32> Rem<Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn rem(self, rhs: Cyclic<N>) -> Self::Output {
        self / rhs
    }
}

impl<const N: u32> Rem<&Cyclic<N>> for Cyclic<N> {
    type Output = Cyclic<N>;
    fn rem(self, rhs: &Cyclic<N>) -> Self::Output {
        self / rhs
    }
}

impl<const N: u32> Rem<Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn rem(self, rhs: Cyclic<N>) -> Self::Output {
        self / rhs
    }
}

impl<const N: u32> Rem<&Cyclic<N>> for &Cyclic<N> {
    type Output = Cyclic<N>;
    fn rem(self, rhs: &Cyclic<N>) -> Self::Output {
        self / rhs
    }
}

impl<const N: u32> Num for Cyclic<N> {
    type FromStrRadixErr = String;
    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> std::prelude::v1::Result<Self, Self::FromStrRadixErr> {
        let Ok(val) = i64::from_str_radix(str, radix) else {
            return Err("Failed to parse number with given radix".to_string());
        };
        Ok(Cyclic {
            val: val.rem_euclid(N as i64) as u32,
        })
    }
}

impl<const N: u32> MulAssign for Cyclic<N> {
    fn mul_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.val) % N
    }
}

impl<const N: u32> DivAssign for Cyclic<N> {
    fn div_assign(&mut self, rhs: Self) {
        self.val = (self.val * rhs.inverse().unwrap().val) % N
    }
}

impl<const N: u32> SubAssign for Cyclic<N> {
    fn sub_assign(&mut self, rhs: Self) {
        self.val = (N + self.val - rhs.val) % N
    }
}

impl<const N: u32> AddAssign for Cyclic<N> {
    fn add_assign(&mut self, rhs: Self) {
        self.val = (self.val + rhs.val) % N
    }
}

impl<const N: u32> MulAdd for Cyclic<N> {
    type Output = Cyclic<N>;
    fn mul_add(self, a: Self, b: Self) -> Self::Output {
        Cyclic {
            val: self.val.mul_add(a.val, b.val) % N,
        }
    }
}

impl<const N: u32> Display for Cyclic<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl<const N: u32> Neg for Cyclic<N> {
    type Output = Cyclic<N>;
    fn neg(self) -> Self::Output {
        Cyclic { val: N - self.val }
    }
}
