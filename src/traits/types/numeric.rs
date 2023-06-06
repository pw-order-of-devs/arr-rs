use std::fmt::{Debug, Display};
use std::ops::RangeInclusive;
use std::str::FromStr;

use rand::Rng;
use rand::distributions::Uniform;

/// Numeric type for array
pub trait Numeric:
Copy + Clone + PartialEq + PartialOrd + Debug + Display + FromStr {
    /// Zero constant value
    const ZERO: Self;
    /// One constant value
    const ONE: Self;

    /// Generate random value
    fn rand(range: RangeInclusive<Self>) -> Self;

    /// Convert from any other numeric
    fn from<U: Numeric>(value: U) -> Self {
        Self::from_f64(value.to_f64())
    }

    /// Convert from usize
    fn from_usize(value: usize) -> Self;
    /// Convert from f64
    fn from_f64(value: f64) -> Self;

    /// Convert to usize
    fn to_usize(&self) -> usize;
    /// Convert to i32
    fn to_i32(&self) -> i32;
    /// Convert to f64
    fn to_f64(&self) -> f64;

    /// bitwise and operation
    fn bitwise_and(&self, other: &Self) -> Self;
    /// bitwise or operation
    fn bitwise_or(&self, other: &Self) -> Self;
    /// bitwise xor operation
    fn bitwise_xor(&self, other: &Self) -> Self;
    /// bitwise not operation
    fn bitwise_not(&self) -> Self;
}

macro_rules! impl_numeric {
    ($t:ty) => {
        impl Numeric for $t {
            const ZERO: Self = 0 as $t;
            const ONE: Self = 1 as $t;

            fn rand(range: RangeInclusive<Self>) -> $t {
                let mut rng = rand::thread_rng();
                let value = rng.sample(&Uniform::from(range));
                value as $t
            }

            fn from_usize(value: usize) -> $t {
                value as $t
            }

            fn from_f64(value: f64) -> $t {
                value as $t
            }

            fn to_usize(&self) -> usize {
                *self as usize
            }

            fn to_i32(&self) -> i32 {
                *self as i32
            }

            fn to_f64(&self) -> f64 {
                *self as f64
            }

            fn bitwise_and(&self, other: &Self) -> Self {
                self & other
            }

            fn bitwise_or(&self, other: &Self) -> Self {
                self | other
            }

            fn bitwise_xor(&self, other: &Self) -> Self {
                self ^ other
            }

            fn bitwise_not(&self) -> Self {
                !self.clone()
            }
        }
    };
}
impl_numeric!(isize);
impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(usize);
impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);

macro_rules! impl_numeric_float {
    ($t:ty) => {
        impl Numeric for $t {
            const ZERO: Self = 0 as $t;
            const ONE: Self = 1 as $t;

            fn rand(range: RangeInclusive<Self>) -> $t {
                let mut rng = rand::thread_rng();
                let value = rng.sample(&Uniform::from(range));
                value as $t
            }

            fn from_usize(value: usize) -> $t {
                value as $t
            }

            fn from_f64(value: f64) -> $t {
                value as $t
            }

            fn to_usize(&self) -> usize {
                *self as usize
            }

            fn to_i32(&self) -> i32 {
                *self as i32
            }

            fn to_f64(&self) -> f64 {
                *self as f64
            }

            fn bitwise_and(&self, other: &Self) -> Self {
                (*self as i128 & *other as i128) as $t
            }

            fn bitwise_or(&self, other: &Self) -> Self {
                (*self as i128 | *other as i128) as $t
            }

            fn bitwise_xor(&self, other: &Self) -> Self {
                (*self as i128 ^ *other as i128) as $t
            }

            fn bitwise_not(&self) -> Self {
                !(*self as i128) as $t
            }
        }
    };
}

impl_numeric_float!(f32);
impl_numeric_float!(f64);

impl Numeric for bool {
    const ZERO: Self = false;
    const ONE: Self = true;

    fn rand(_: RangeInclusive<Self>) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen::<bool>()
    }

    fn from_usize(value: usize) -> Self {
        value == 1
    }

    fn from_f64(value: f64) -> Self {
        value == 1.
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_f64(&self) -> f64 {
        self.to_usize() as f64
    }

    fn bitwise_and(&self, other: &Self) -> Self {
        self & other
    }

    fn bitwise_or(&self, other: &Self) -> Self {
        self | other
    }

    fn bitwise_xor(&self, other: &Self) -> Self {
        self ^ other
    }

    fn bitwise_not(&self) -> Self {
        !*self
    }
}
