/// Collection types definition
pub mod collection;

/// Tuple types definition
pub mod tuple;

use std::fmt::{Debug, Display};

/// base trait for arrays
pub trait ArrayElement: Clone + Display + Debug + PartialEq + PartialOrd {
    /// Zero constant value
    fn zero() -> Self;
    /// One constant value
    fn one() -> Self;
}
