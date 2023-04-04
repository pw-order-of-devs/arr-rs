// traits traits
pub use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    manipulate::ArrayManipulate,
    math::ArrayMath,
    meta::ArrayMeta,
    types::{
        Numeric,
        SignedNumeric,
    },
};

// implementations
pub use crate::arrays::Array;

// macros
#[cfg(feature = "macros")]
pub use crate::{
    array,
    array_rand,
    array_flat,
    array_zeros,
    array_ones,
};
