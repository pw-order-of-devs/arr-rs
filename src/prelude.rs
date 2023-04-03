// traits traits
pub use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    manipulate::ArrayManipulate,
    math::ArrayMath,
    meta::ArrayMeta,
    types::Numeric,
};

// implementations
pub use crate::arrays::Array;

// macros
#[cfg(feature = "macros")]
pub use crate::array;
