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

/// Tuple numeric types definition
pub mod tuple_numeric;

/// base trait for arrays
pub trait ArrayElement: Clone + std::fmt::Display + std::fmt::Debug {}
