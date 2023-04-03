use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    RangeInclusive,
    Rem, RemAssign,
    Sub, SubAssign,
};

use rand::Rng;
use rand::distributions::Uniform;

/// Base numeric type for array
pub trait Numeric:
Copy + Clone + PartialEq + PartialOrd + std::fmt::Display +
Add<Self, Output=Self> + AddAssign<Self> +
Sub<Self, Output=Self> + SubAssign<Self> +
Mul<Self, Output=Self> + MulAssign<Self> +
Div<Self, Output=Self> + DivAssign<Self> +
Rem<Self, Output=Self> + RemAssign<Self> {
    /// Zero constant value
    const ZERO: Self;
    /// One constant value
    const ONE: Self;
    /// Generate random value
    fn rand(range: RangeInclusive<Self>) -> Self;
}

macro_rules! impl_zero_one_numeric {
    ($t:ty) => {
        impl Numeric for $t {
            const ZERO: Self = 0 as $t;
            const ONE: Self = 1 as $t;

            fn rand(range: RangeInclusive<Self>) -> $t {
                let mut rng = rand::thread_rng();
                let value = rng.sample(&Uniform::from(range));
                value as $t
            }
        }
    };
}

impl_zero_one_numeric!(f32);
impl_zero_one_numeric!(f64);
impl_zero_one_numeric!(i8);
impl_zero_one_numeric!(i16);
impl_zero_one_numeric!(i32);
impl_zero_one_numeric!(i64);
impl_zero_one_numeric!(u8);
impl_zero_one_numeric!(u16);
impl_zero_one_numeric!(u32);
impl_zero_one_numeric!(u64);
