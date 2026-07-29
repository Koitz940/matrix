use core::num;
use num_traits::{MulAdd, Num};
use num_traits::{One, Signed, Zero};
use std::fmt::{Debug, Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

pub type Comp = Complex<f64>; //default
pub type C64 = Complex<f64>; //systematic name for default
pub type C32 = Complex<f32>; //f32
pub type Cg8 = Complex<i8>; //Gaussian Integer i8
pub type Cg16 = Complex<i16>; //Gaussian Integer i16
pub type Cg32 = Complex<i32>; //Gaussian Integer i32
pub type Cg64 = Complex<i64>; //Gaussian integer i64
pub type Cg128 = Complex<i128>; //Gaussian integer i128
pub type Cs = Complex<String>; //Complex string, idk why it's funny as all hell, tho well, only adding will work lol

impl<T: Mul<Output = T> + Add<Output = T> + Copy> Complex<T> {
    pub fn modl(&self) -> T {
        self.re * self.re + self.im * self.im
    }
}

impl<T: Neg<Output = T> + Copy> Complex<T> {
    pub fn comp(&self) -> Complex<T> {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }
}

impl<T: Add<Output = T>> Add<T> for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re + rhs,
            im: self.im,
        }
    }
}

impl<T: Add<Output = T>> Add for Complex<T> {
    type Output = Complex<T>;
    fn add(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl<T: AddAssign> AddAssign for Complex<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.im += rhs.im;
        self.re += rhs.re;
    }
}

impl<T: AddAssign> AddAssign<T> for Complex<T> {
    fn add_assign(&mut self, rhs: T) {
        self.re += rhs;
    }
}

impl<T: Sub<Output = T>> Sub<T> for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re - rhs,
            im: self.im,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Complex<T> {
    type Output = Complex<T>;
    fn sub(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl<T: SubAssign> SubAssign for Complex<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.im -= rhs.im;
        self.re -= rhs.re;
    }
}

impl<T: SubAssign> SubAssign<T> for Complex<T> {
    fn sub_assign(&mut self, rhs: T) {
        self.re -= rhs;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> Mul for Complex<T> {
    type Output = Complex<T>;
    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.im * rhs.re + self.re * rhs.im,
        }
    }
}

impl<T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Copy> MulAssign for Complex<T> {
    fn mul_assign(&mut self, rhs: Self) {
        let im = self.im * rhs.re + self.re * rhs.im;
        let re = self.re * rhs.re - self.im * rhs.im;
        self.re = re;
        self.im = im;
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Complex<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.re *= rhs;
        self.im *= rhs;
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Complex<T> {
    type Output = Complex<T>;
    fn div(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl<T> Div<Complex<T>> for Complex<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Neg<Output = T>,
    Complex<T>: Mul<Complex<T>, Output = Complex<T>> + Div<T, Output = Complex<T>>,
{
    type Output = Complex<T>;
    fn div(self, rhs: Complex<T>) -> Complex<T> {
        let m = rhs.modl();
        (self * rhs.comp()) / m
    }
}

impl<
    T: Copy + Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Sub<Output = T> + Div<Output = T>,
> DivAssign for Complex<T>
{
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl<T: DivAssign + Copy> DivAssign<T> for Complex<T> {
    fn div_assign(&mut self, rhs: T) {
        self.re /= rhs;
        self.im /= rhs;
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} + {}i", self.re, self.im)
    }
}

impl<T: Default> Default for Complex<T> {
    fn default() -> Self {
        Complex {
            re: T::default(),
            im: T::default(),
        }
    }
}

impl<T: Zero + One + PartialEq + Sub<Output = T> + Copy> One for Complex<T> {
    fn one() -> Self {
        Complex {
            re: T::one(),
            im: T::zero(),
        }
    }

    fn is_one(&self) -> bool {
        *self
            == Complex {
                re: T::one(),
                im: T::zero(),
            }
    }

    fn set_one(&mut self) {
        self.im = T::zero();
        self.re = T::one();
    }
}

impl<T: Zero + PartialEq> Zero for Complex<T> {
    fn zero() -> Self {
        Complex {
            re: T::zero(),
            im: T::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        *self
            == Complex {
                re: T::zero(),
                im: T::zero(),
            }
    }

    fn set_zero(&mut self) {
        self.im = T::zero();
        self.re = T::zero();
    }
}

impl<T: Neg<Output = T>> Neg for Complex<T> {
    type Output = Complex<T>;
    fn neg(self) -> Complex<T> {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl<T, R, S> MulAdd<R, S> for Complex<T>
where
    T: Sub<Output = T> + Copy + Add<Output = T> + Mul<T, Output = T>,
    Complex<T>: Mul<R, Output = Complex<T>> + Add<S, Output = Complex<T>>,
{
    type Output = Complex<T>;

    fn mul_add(self, a: R, b: S) -> Complex<T> {
        (self * a) + b
    }
}

impl<T: Copy + PartialOrd + Mul<Output = T> + Add<Output = T>> Complex<T> {
    fn modl_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.modl().partial_cmp(&other.modl())
    }
}

pub trait RealMagnitude {
    type Real;
    fn abs_val(&self) -> Self::Real;
}

impl<T: Signed> RealMagnitude for T {
    type Real = T;
    fn abs_val(&self) -> T {
        self.abs()
    }
}

impl<T: Copy + num_traits::Float> RealMagnitude for Complex<T> {
    type Real = T;
    fn abs_val(&self) -> T {
        (self.re * self.re + self.im * self.im).sqrt()
    }
}

impl<T: Neg<Output = T> + Copy> Complex<T> {
    pub fn conj(&self) -> Complex<T> {
        Complex {
            re: self.re,
            im: -self.im,
        }
    }

    pub fn make_conj(&mut self) {
        self.im = -self.im
    }
}

impl<
    T: Copy + Sub<Output = T> + Mul<Output = T> + Neg<Output = T> + Add<Output = T> + Div<Output = T>,
> Rem for Complex<T>
{
    type Output = Complex<T>;
    fn rem(self, rhs: Self) -> Self::Output {
        return self - (self / rhs);
    }
}

impl<
    T: Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Neg<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Zero
        + One
        + PartialEq
        + Num,
> Num for Complex<T>
{
    type FromStrRadixErr = String;
    fn from_str_radix(
        str: &str,
        radix: u32,
    ) -> std::prelude::v1::Result<Self, Self::FromStrRadixErr> {
        if str.chars().filter(|&c| c == '+').count() != 1
            && str.chars().filter(|&c| c.is_whitespace()).count() != 0
        {
            return Err(String::from(str.to_string() + "not a complex number"));
        } else {
            let numbers: Vec<&str> = str.split(',').collect();
            if numbers.len() != 2 {
                return Err(String::from(str.to_string() + "is not a complex number"));
            }
            let re = match T::from_str_radix(numbers[0], radix) {
                Ok(num) => num,
                Err(_) => return Err("Real part not a number".to_string()),
            };
            if numbers[1].len() < 2 {
                return Err("bad imagianry part".to_string());
            }
            let mut imm = numbers[1].chars();
			imm.next_back();
            let im = match T::from_str_radix(imm.as_str(), radix) {
                Ok(num) => num,
                Err(_) => return Err("Real part not a number".to_string()),
            };
            Ok(Complex { re: re, im: im })
        }
    }
}
