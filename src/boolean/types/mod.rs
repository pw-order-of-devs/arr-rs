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
}

impl Numeric for bool {

    fn rand(_: RangeInclusive<Self>) -> Self {
        let mut rng = rand::thread_rng();
        rng.gen::<bool>()
    }

    fn from_usize(value: usize) -> Self {
        value == 1
    }

    fn from_f64(value: f64) -> Self {
        value == 1.
    }

    fn to_usize(&self) -> usize {
        *self as usize
    }

    fn to_i32(&self) -> i32 {
        *self as i32
    }

    fn to_f64(&self) -> f64 {
        self.to_usize() as f64
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
        format!("{:b}", *self as usize)
    }
}

impl BoolNumeric for bool {}
