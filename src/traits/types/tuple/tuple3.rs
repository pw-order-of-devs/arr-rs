use std::fmt::Display;
use std::str::FromStr;

use crate::traits::types::{
    ArrayElement,
    tuple::{
        ParseTupleError,
        TupleElement,
    },
};

/// Tuple2 type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Tuple3<T: ArrayElement>(pub T, pub T, pub T);

impl<T: ArrayElement + FromStr> FromStr for Tuple3<T> {
    type Err = ParseTupleError<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let mut parts = s.split(", ");
        let x = parts.next().ok_or(ParseTupleError::Format)?;
        let y = parts.next().ok_or(ParseTupleError::Format)?;
        let z = parts.next().ok_or(ParseTupleError::Format)?;

        let x = T::from_str(x).map_err(ParseTupleError::Parse)?;
        let y = T::from_str(y).map_err(ParseTupleError::Parse)?;
        let z = T::from_str(z).map_err(ParseTupleError::Parse)?;

        Ok(Tuple3(x, y, z))
    }
}

impl <T: ArrayElement> ArrayElement for Tuple3<T> {

    fn zero() -> Self {
        Tuple3(T::zero(), T::zero(), T::zero())
    }

    fn one() -> Self {
        Tuple3(T::one(), T::one(), T::one())
    }
}

impl <T: ArrayElement> Display for Tuple3<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl <T: ArrayElement> TupleElement<T> for Tuple3<T> {
    type Input = (T, T, T);
    type Output = Self;

    fn from_tuple(tuple: (T, T, T)) -> Self::Output {
        Tuple3(tuple.0, tuple.1, tuple.2)
    }
}
