use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Rem, RemAssign,
    Sub, SubAssign,
};

use crate::traits::types::numeric::Numeric;

/// Numeric Ops type for array
pub trait NumericOps: Numeric +
Add<Self, Output=Self> + AddAssign<Self> +
Sub<Self, Output=Self> + SubAssign<Self> +
Mul<Self, Output=Self> + MulAssign<Self> +
Div<Self, Output=Self> + DivAssign<Self> +
Rem<Self, Output=Self> + RemAssign<Self> {}

macro_rules! impl_numeric_ops {
    ($t:ty) => {
        impl NumericOps for $t {}
    };
}

impl_numeric_ops!(f32);
impl_numeric_ops!(f64);
impl_numeric_ops!(i8);
impl_numeric_ops!(i16);
impl_numeric_ops!(i32);
impl_numeric_ops!(i64);
