// traits traits
pub use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    manipulate::ArrayManipulate,
    math::ArrayMath,
    meta::ArrayMeta,
    types::{
        Numeric,
        NumericOps,
        SignedNumeric,
        BoolNumeric,
    },
};

// implementations
pub use crate::arrays::Array;

// macros
#[cfg(feature = "macros")]
pub use crate::{
    array,
    array_arange,
    array_rand,
    array_flat,
    array_eye,
    array_identity,
    array_zeros,
    array_ones,
    array_full,
};
