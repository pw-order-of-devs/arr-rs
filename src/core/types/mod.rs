/// Collection types definition
pub mod collection;

/// Sort parameters types definition
pub mod sort;

/// Tuple types definition
pub mod tuple;

use std::fmt::{Debug, Display};

/// base trait for arrays
pub trait ArrayElement: Clone + Display + Debug + PartialEq + PartialOrd {
    /// Zero constant value
    fn zero() -> Self;
    /// One constant value
    fn one() -> Self;
    /// Is NAN
    fn is_nan(&self) -> bool;
}
