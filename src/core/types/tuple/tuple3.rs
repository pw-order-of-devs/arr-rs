use std::fmt::Display;
use std::str::FromStr;

use crate::core::prelude::*;

pub(crate) type TupleH3 <S, T, U> = (Array<S>, Array<T>, Array<U>);

/// Tuple3 type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Tuple3<S: ArrayElement, T: ArrayElement, U: ArrayElement>(pub S, pub T, pub U);

impl<S: ArrayElement + FromStr, T: ArrayElement + FromStr, U: ArrayElement + FromStr> FromStr for Tuple3<S, T, U>
    where <S as FromStr>::Err: std::fmt::Debug,
          <T as FromStr>::Err: std::fmt::Debug,
          <U as FromStr>::Err: std::fmt::Debug, {
    type Err = ParseTupleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim_start_matches('(')
            .trim_end_matches(')')
            .replace(", ", ",");
        let mut parts = s.split(',');

        let x = parts.next().ok_or(ParseTupleError::Format)?;
        let y = parts.next().ok_or(ParseTupleError::Format)?;
        let z = parts.next().ok_or(ParseTupleError::Format)?;

        let x = S::from_str(x);
        let y = T::from_str(y);
        let z = U::from_str(z);

        if x.is_err() || y.is_err() || z.is_err() {
            return Err(ParseTupleError::Parse("error parsing tuple value"))
        }

        Ok(Tuple3(x.unwrap(), y.unwrap(), z.unwrap()))
    }
}

impl <S: ArrayElement, T: ArrayElement, U: ArrayElement> ArrayElement for Tuple3<S, T, U> {

    fn zero() -> Self {
        Tuple3(S::zero(), T::zero(), U::zero())
    }

    fn one() -> Self {
        Tuple3(S::one(), T::one(), U::one())
    }

    fn is_nan(&self) -> bool {
        self.0.is_nan() || self.1.is_nan() || self.2.is_nan()
    }
}

impl <S: ArrayElement, T: ArrayElement, U: ArrayElement> Display for Tuple3<S, T, U> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.0, self.1, self.2)
    }
}

impl <S: ArrayElement, T: ArrayElement, U: ArrayElement> TupleElement<T> for Tuple3<S, T, U> {
    type Input = (S, T, U);
    type Output = Self;

    fn from_tuple(tuple: (S, T, U)) -> Self::Output {
        Tuple3(tuple.0, tuple.1, tuple.2)
    }
}
