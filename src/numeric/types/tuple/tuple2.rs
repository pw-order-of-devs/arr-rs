use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::{
    core::prelude::*,
    numeric::prelude::*,
};

impl<T: Numeric + FromStr> FromStr for Tuple2<T> {
    type Err = ParseTupleError<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let mut parts = s.split(", ");
        let x = parts.next().ok_or(ParseTupleError::Format)?;
        let y = parts.next().ok_or(ParseTupleError::Format)?;

        let x = T::from_str(x).map_err(ParseTupleError::Parse)?;
        let y = T::from_str(y).map_err(ParseTupleError::Parse)?;

        Ok(Tuple2(x, y))
    }
}

impl <N: Numeric> Numeric for Tuple2<N> {
    fn rand(range: RangeInclusive<Self>) -> Self {
        let start = range.start();
        let end = range.end();
        Tuple2(
            Numeric::rand(RangeInclusive::new(start.0, end.0)),
            Numeric::rand(RangeInclusive::new(start.1, end.1)),
        )
    }

    fn from_usize(value: usize) -> Self {
        Tuple2(Numeric::from_usize(value), Numeric::from_usize(value))
    }

    fn from_f64(value: f64) -> Self {
        Tuple2(Numeric::from_f64(value), Numeric::from_f64(value))
    }

    fn to_usize(&self) -> usize {
        self.0.to_usize()
    }

    fn to_i32(&self) -> i32 {
        self.0.to_i32()
    }

    fn to_f64(&self) -> f64 {
        self.0.to_f64()
    }

    fn bitwise_and(&self, other: &Self) -> Self {
        Self(
            self.0.bitwise_and(&other.0),
            self.1.bitwise_and(&other.1),
        )
    }

    fn bitwise_or(&self, other: &Self) -> Self {
        Self(
            self.0.bitwise_or(&other.0),
            self.1.bitwise_or(&other.1),
        )
    }

    fn bitwise_xor(&self, other: &Self) -> Self {
        Self(
            self.0.bitwise_xor(&other.0),
            self.1.bitwise_xor(&other.1),
        )
    }

    fn bitwise_not(&self) -> Self {
        Self(
            self.0.bitwise_not(),
            self.1.bitwise_not(),
        )
    }

    fn left_shift(&self, other: &Self) -> Self {
        Self(
            self.0.left_shift(&other.0),
            self.1.left_shift(&other.1),
        )
    }

    fn right_shift(&self, other: &Self) -> Self {
        Self(
            self.0.right_shift(&other.0),
            self.1.right_shift(&other.1),
        )
    }

    fn binary_repr(&self) -> String {
        format!("({}, {})", self.0.binary_repr(), self.1.binary_repr())
    }
}

impl <N: Numeric> From<(N, N)> for Tuple2<N> {

    fn from(value: (N, N)) -> Self {
        Tuple2(value.0, value.1)
    }
}
