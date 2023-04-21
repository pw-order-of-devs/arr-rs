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
    /// Convert to f64
    fn to_f64(&self) -> f64;
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

            fn to_f64(&self) -> f64 {
                *self as f64
            }
        }
    };
}

impl_numeric!(f32);
impl_numeric!(f64);
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

    fn to_f64(&self) -> f64 {
        self.to_usize() as f64
    }
}
