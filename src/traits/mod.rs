/// Array Create functions
pub mod create;
/// Array Indexing and Slicing functions
pub mod indexing;
/// Array Manipulate functions
pub mod manipulate;
/// Array Math functions
pub mod math;
/// Array Metadata functions
pub mod meta;

/// Numeric type for which array is implemented
pub mod types;

use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    manipulate::ArrayManipulate,
    math::ArrayMath,
    meta::ArrayMeta,
    types::Numeric,
};

/// Definition of Array
///
/// This trait is implemented by Array, assuring the functionalities:
///
/// * ArrayCreate     - create array functions
/// * ArrayIndexing   - indexing and slicing functions of array
/// * ArrayManipulate - manipulation of array content
/// * ArrayMath       - mathematical functions on array
/// * ArrayMeta       - metadata of array
///
pub trait ArrayTrait<N: Numeric>
    where Self: Sized + Clone + std::fmt::Display + FromIterator<N> + IntoIterator<Item=N> +
    ArrayCreate<N> + ArrayIndexing<N> + ArrayManipulate<N> + ArrayMath<N> + ArrayMeta<N> {}
