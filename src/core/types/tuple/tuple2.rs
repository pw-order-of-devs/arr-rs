use std::fmt::Display;

use crate::core::prelude::*;

/// Tuple2 type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Tuple2<T: ArrayElement>(pub T, pub T);

impl <T: ArrayElement> ArrayElement for Tuple2<T> {

    fn zero() -> Self {
        Tuple2(T::zero(), T::zero())
    }

    fn one() -> Self {
        Tuple2(T::one(), T::one())
    }
}

impl <T: ArrayElement> Display for Tuple2<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl <T: ArrayElement> TupleElement<T> for Tuple2<T> {
    type Input = (T, T);
    type Output = Self;

    fn from_tuple(tuple: (T, T)) -> Self::Output {
        Tuple2(tuple.0, tuple.1)
    }
}
