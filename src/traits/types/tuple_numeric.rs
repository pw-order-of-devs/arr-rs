use std::fmt::Display;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::traits::types::numeric::Numeric;

/// Numeric Tuple trait for array
pub trait TupleNumeric<N: Numeric>: Numeric {
    /// Output type for TupleNumeric
    type Output;

    /// parse numeric type from tuple
    fn from_tuple(tuple: (N, N)) -> Self::Output;
}

/// Numeric Tuple type for array
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Tuple2<N: Numeric>(pub N, pub N);

/// Error definition for tuple parsing
#[derive(Debug)]
pub enum ParseTuple2Error<T: FromStr> {
    /// Error definition for tuple parsing - Parse error
    Parse(T::Err),
    /// Error definition for tuple parsing - Format error
    Format,
}

impl<T: FromStr> Display for ParseTuple2Error<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseTuple2Error::Parse(_) => write!(f, "Parse error"),
            ParseTuple2Error::Format => write!(f, "Format error"),
        }
    }
}

impl<T: Numeric + FromStr> FromStr for Tuple2<T> {
    type Err = ParseTuple2Error<T>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{s}");
        let s = s.trim_start_matches('(').trim_end_matches(')');
        let mut parts = s.split(", ");
        let x = parts.next().ok_or(ParseTuple2Error::Format)?;
        let y = parts.next().ok_or(ParseTuple2Error::Format)?;

        let x = T::from_str(x).map_err(ParseTuple2Error::Parse)?;
        let y = T::from_str(y).map_err(ParseTuple2Error::Parse)?;

        Ok(Tuple2(x, y))
    }
}

impl <N: Numeric> Numeric for Tuple2<N> {
    const ZERO: Self = Tuple2(N::ZERO, N::ZERO);
    const ONE: Self = Tuple2(N::ONE, N::ONE);

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
}

impl<N: Numeric> Display for Tuple2<N> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl <N: Numeric> From<(N, N)> for Tuple2<N> {

    fn from(value: (N, N)) -> Self {
        Tuple2(value.0, value.1)
    }
}

impl <N: Numeric> TupleNumeric<N> for Tuple2<N> {
    type Output = Self;

    fn from_tuple(tuple: (N, N)) -> Self::Output {
        Tuple2(tuple.0, tuple.1)
    }
}
