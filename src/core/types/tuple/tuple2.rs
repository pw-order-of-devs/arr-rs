use std::fmt::Display;
use std::str::FromStr;

use crate::core::prelude::*;

pub(crate) type TupleH2 <T, S> = (Array<T>, Array<S>);

/// Tuple2 type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Tuple2<S: ArrayElement, T: ArrayElement>(pub S, pub T);

impl<S: ArrayElement + FromStr, T: ArrayElement + FromStr> FromStr for Tuple2<S, T>
    where <S as FromStr>::Err: std::fmt::Debug,
          <T as FromStr>::Err: std::fmt::Debug, {
    type Err = ParseTupleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim_start_matches('(')
            .trim_end_matches(')')
            .replace(", ", ",");
        let mut parts = s.split(',');
        let x = parts.next().ok_or(ParseTupleError::Format)?;
        let y = parts.next().ok_or(ParseTupleError::Format)?;

        let x = S::from_str(x);
        let y = T::from_str(y);

        if x.is_err() || y.is_err() {
            return Err(ParseTupleError::Parse("error parsing tuple value"))
        }

        Ok(Self(x.unwrap(), y.unwrap()))
    }
}

impl <S: ArrayElement, T: ArrayElement> ArrayElement for Tuple2<S, T> {

    fn zero() -> Self {
        Self(S::zero(), T::zero())
    }

    fn one() -> Self {
        Self(S::one(), T::one())
    }

    fn is_nan(&self) -> bool {
        self.0.is_nan() || self.1.is_nan()
    }
}

impl <S: ArrayElement, T: ArrayElement> Display for Tuple2<S, T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl <S: ArrayElement, T: ArrayElement> TupleElement<T> for Tuple2<S, T> {
    type Input = (S, T);
    type Output = Self;

    fn from_tuple(tuple: (S, T)) -> Self::Output {
        Self(tuple.0, tuple.1)
    }
}
