// base traits
pub use crate::base::{
    base_array::ArrayBase,
    base_type::Numeric,
};

// implementations
pub use crate::arrays::{
    array::Array,
};

#[cfg(feature = "macros")]
pub use crate::array;
