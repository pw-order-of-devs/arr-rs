/// create array functions implementation
pub mod create;
/// indexing and slicing array functions implementation
pub mod indexing;
/// manipulate array functions implementation
pub mod manipulate;
/// math array functions implementation
pub mod math;
/// metadata of array
pub mod meta;

/// binary array operations implementation
pub mod binary;

/// implementations of external traits
pub mod impls;

/// create array macro implementation
#[cfg(feature = "macros")]
pub mod macros;

use crate::traits::types::numeric::Numeric;

/// Array structure definition
#[derive(Clone, Debug)]
pub struct Array<N: Numeric> {
    pub(crate) elements: Vec<N>,
    pub(crate) shape: Vec<usize>,
}
