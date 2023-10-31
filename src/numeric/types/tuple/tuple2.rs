use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::{
    core::prelude::*,
    numeric::prelude::*,
};

impl <M: Numeric, N: Numeric> Numeric for Tuple2<M, N>
    where <M as FromStr>::Err: std::fmt::Debug,
          <N as FromStr>::Err: std::fmt::Debug, {
    fn rand(range: RangeInclusive<Self>) -> Self {
        let start = range.start();
        let end = range.end();
        Self(
            Numeric::rand(RangeInclusive::new(start.0, end.0)),
            Numeric::rand(RangeInclusive::new(start.1, end.1)),
        )
    }

    fn from_usize(value: usize) -> Self {
        Self(Numeric::from_usize(value), Numeric::from_usize(value))
    }

    fn from_f64(value: f64) -> Self {
        Self(Numeric::from_f64(value), Numeric::from_f64(value))
    }

    fn to_usize(&self) -> usize {
        self.0.to_usize()
    }

    fn to_isize(&self) -> isize {
        self.0.to_isize()
    }

    fn to_i32(&self) -> i32 {
        self.0.to_i32()
    }

    fn to_f64(&self) -> f64 {
        self.0.to_f64()
    }

    fn is_inf(&self) -> bool {
        self.0.is_inf() || self.1.is_inf()
    }

    fn max(&self) -> Self {
        Self(self.0.max(), self.1.max())
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

impl <M: Numeric, N: Numeric> From<(M, N)> for Tuple2<M, N> {

    fn from(value: (M, N)) -> Self {
        Self(value.0, value.1)
    }
}
