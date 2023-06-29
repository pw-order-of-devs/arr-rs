/// Alphanumeric type definition
pub mod alphanumeric;

/// BoolNumeric type definition
pub mod bool_numeric;

/// Numeric type definition
pub mod numeric;

/// NumericOps type definition
pub mod numeric_ops;

/// SignedNumeric type definition
pub mod signed_numeric;

/// Tuple types definition
pub mod tuple;

/// Collection types definition
pub mod collection;

use std::fmt::{Debug, Display};

/// base trait for arrays
pub trait ArrayElement: Clone + Display + Debug + PartialEq + PartialOrd {
    /// Zero constant value
    fn zero() -> Self;
    /// One constant value
    fn one() -> Self;
}
