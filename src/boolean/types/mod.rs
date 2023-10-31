use std::ops::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Not,
    RangeInclusive,
};
use rand::Rng;

use crate::{
    core::prelude::*,
    numeric::prelude::*,
};

/// Boolean Numeric type for array
pub trait BoolNumeric: Numeric + Not +
BitAnd + BitAndAssign +
BitOr + BitOrAssign +
BitXor + BitXorAssign {}

impl ArrayElement for bool {

    fn zero() -> Self {
        false
    }

    fn one() -> Self {
        true
    }

    fn is_nan(&self) -> bool {
        false
    }
}

impl Numeric for bool {

    fn rand(_: RangeInclusive<Self>) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen::<Self>()
    }

    fn from_usize(value: usize) -> Self {
        value == 1
    }

    fn from_f64(value: f64) -> Self {
        (value - 1.).abs() < 1e-24
    }

    fn to_usize(&self) -> usize {
        <usize as From<_>>::from(*self)
    }

    fn to_isize(&self) -> isize {
        <isize as From<_>>::from(*self)
    }

    fn to_i32(&self) -> i32 {
        <i32 as From<_>>::from(*self)
    }

    fn to_f64(&self) -> f64 {
        <f64 as From<_>>::from(<u16 as From<_>>::from(*self))
    }

    fn is_inf(&self) -> bool {
        false
    }

    fn max(&self) -> Self {
        true
    }

    fn bitwise_and(&self, other: &Self) -> Self {
        self & other
    }

    fn bitwise_or(&self, other: &Self) -> Self {
        self | other
    }

    fn bitwise_xor(&self, other: &Self) -> Self {
        self ^ other
    }

    fn bitwise_not(&self) -> Self {
        !*self
    }

    fn left_shift(&self, other: &Self) -> Self {
        (self.to_usize() << other.to_usize()) == 1
    }

    fn right_shift(&self, other: &Self) -> Self {
        (self.to_usize() >> other.to_usize()) == 1
    }

    fn binary_repr(&self) -> String {
        format!("{:b}", self.to_usize())
    }
}

impl BoolNumeric for bool {}
