#[cfg(feature = "numeric")]
pub use crate::numeric::{
    operations::{
        binary::ArrayBinary,
        binary_bits::ArrayBinaryBits,
        create::ArrayCreateNumeric,
        create_from::ArrayCreateFrom,
    },
    types::{
        binary::{BitOrder, BitOrderType},
        floating::Floating,
        numeric::Numeric,
        numeric_ops::NumericOps,
        signed_ops::SignedNumericOps,
    },
};
