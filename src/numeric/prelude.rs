#[cfg(feature = "numeric")]
pub use crate::numeric::{
    operations::{
        binary::ArrayBinary,
        binary_bits::ArrayBinaryBits,
        create::ArrayCreateNumeric,
        create_from::ArrayCreateFrom,
        math::ArrayMath,
    },
    types::{
        binary::BitOrder,
        numeric::Numeric,
        numeric_ops::NumericOps,
        signed_ops::SignedNumericOps,
    },
};
