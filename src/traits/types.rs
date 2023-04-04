use std::ops::{
    Add, AddAssign,
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Neg, Not,
    RangeInclusive,
    Rem, RemAssign,
    Sub, SubAssign,
};

use rand::Rng;
use rand::distributions::Uniform;

/// Numeric type for array
pub trait Numeric:
Copy + Clone + PartialEq + PartialOrd + std::fmt::Display {
    /// Zero constant value
    const ZERO: Self;
    /// One constant value
    const ONE: Self;
    /// Generate random value
    fn rand(range: RangeInclusive<Self>) -> Self;
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
        }
    };
}

impl_numeric!(f32);
impl_numeric!(f64);
impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
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
}

/// Numeric Ops type for array
pub trait NumericOps: Numeric +
Add<Self, Output=Self> + AddAssign<Self> +
Sub<Self, Output=Self> + SubAssign<Self> +
Mul<Self, Output=Self> + MulAssign<Self> +
Div<Self, Output=Self> + DivAssign<Self> +
Rem<Self, Output=Self> + RemAssign<Self> {}

impl NumericOps for f32 {}
impl NumericOps for f64 {}
impl NumericOps for i8 {}
impl NumericOps for i16 {}
impl NumericOps for i32 {}
impl NumericOps for i64 {}

/// Signed Numeric type for array
pub trait SignedNumeric: NumericOps + Neg<Output=Self> {}

macro_rules! impl_signed_numeric {
    ($t:ty) => {
        impl SignedNumeric for $t {}
    };
}

impl_signed_numeric!(f32);
impl_signed_numeric!(f64);
impl_signed_numeric!(i8);
impl_signed_numeric!(i16);
impl_signed_numeric!(i32);
impl_signed_numeric!(i64);

/// Signed Numeric type for array
pub trait BoolNumeric: Numeric + Not +
BitAnd + BitAndAssign +
BitOr + BitOrAssign +
BitXor + BitXorAssign {}

impl BoolNumeric for bool {}
