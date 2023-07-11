use std::ops::Neg;

use crate::numeric::types::numeric_ops::NumericOps;

/// Signed Numeric type for array
pub trait SignedNumericOps: NumericOps + Neg<Output=Self> {}

macro_rules! impl_signed_numeric {
    ($t:ty) => {
        impl SignedNumericOps for $t {}
    };
}

impl_signed_numeric!(f32);
impl_signed_numeric!(f64);
impl_signed_numeric!(i8);
impl_signed_numeric!(i16);
impl_signed_numeric!(i32);
impl_signed_numeric!(i64);
