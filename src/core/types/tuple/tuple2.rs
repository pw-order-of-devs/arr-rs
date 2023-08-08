use std::fmt::Display;

use crate::core::prelude::*;

pub(crate) type TupleH2 <T, S> = (Array<T>, Array<S>);

/// Tuple2 type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Tuple2<S: ArrayElement, T: ArrayElement>(pub S, pub T);

impl <S: ArrayElement,T: ArrayElement> ArrayElement for Tuple2<S, T> {

    fn zero() -> Self {
        Tuple2(S::zero(), T::zero())
    }

    fn one() -> Self {
        Tuple2(S::one(), T::one())
    }

    fn is_nan(&self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }
}

impl <S: ArrayElement,T: ArrayElement> Display for Tuple2<S, T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl <S: ArrayElement,T: ArrayElement> TupleElement<T> for Tuple2<S, T> {
    type Input = (S, T);
    type Output = Self;

    fn from_tuple(tuple: (S, T)) -> Self::Output {
        Tuple2(tuple.0, tuple.1)
    }
}
