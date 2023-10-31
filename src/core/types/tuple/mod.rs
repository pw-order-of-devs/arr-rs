/// Tuple2 definition
pub mod tuple2;
/// Tuple3 definition
pub mod tuple3;

use crate::core::types::ArrayElement;

/// Generic Tuple trait for array
pub trait TupleElement<T: ArrayElement> {
    /// Input type for `TupleElement`
    type Input;
    /// Output type for `TupleElement`
    type Output;

    /// parse type from tuple
    fn from_tuple(tuple: Self::Input) -> Self::Output;
}

/// Error definition for tuple parsing
#[derive(Debug)]
pub enum ParseTupleError {
    /// Error definition for tuple parsing - Parse error
    Parse(&'static str),
    /// Error definition for tuple parsing - Format error
    Format,
}
