use std::fmt::{Debug, Display};
use std::i64;
use std::ops::RangeInclusive;
use std::str::FromStr;
use rand::{Rng, distributions::Uniform};

use crate::core::types::ArrayElement;

/// Numeric type for array
pub trait Numeric: ArrayElement + Clone + Copy + Display + Debug + PartialEq + PartialOrd + FromStr {

    /// Generate random value
    fn rand(range: RangeInclusive<Self>) -> Self;

    /// Convert from any other numeric
    fn from<U: Numeric>(value: U) -> Self {
        Self::from_f64(value.to_f64())
    }

    /// Convert from usize
    fn from_usize(value: usize) -> Self;
    /// Convert from f64
    fn from_f64(value: f64) -> Self;

    /// Convert to usize
    fn to_usize(&self) -> usize;
    /// Convert to i32
    fn to_i32(&self) -> i32;
    /// Convert to f64
    fn to_f64(&self) -> f64;

    /// Check if is infinity
    fn is_inf(&self) -> bool;
    /// Get max value for type
    fn max(&self) -> Self;

    /// bitwise and operation
    fn bitwise_and(&self, other: &Self) -> Self;
    /// bitwise or operation
    fn bitwise_or(&self, other: &Self) -> Self;
    /// bitwise xor operation
    fn bitwise_xor(&self, other: &Self) -> Self;
    /// bitwise not operation
    fn bitwise_not(&self) -> Self;
    /// left shift operation
    fn left_shift(&self, other: &Self) -> Self;
    /// right shift operation
    fn right_shift(&self, other: &Self) -> Self;
    /// binary representation of number
    fn binary_repr(&self) -> String;
}

macro_rules! impl_numeric {
    ($t:ty) => {
        impl ArrayElement for $t {

            fn zero() -> Self {
                0 as $t
            }

            fn one() -> Self {
                1 as $t
            }

            fn is_nan(&self) -> bool {
                false
            }
        }

        impl Numeric for $t {

            fn rand(range: RangeInclusive<Self>) -> $t {
                let mut rng = rand::thread_rng();
                let value = rng.sample(&Uniform::from(range));
                value as $t
            }

            fn from_usize(value: usize) -> $t {
                value as $t
            }

            fn from_f64(value: f64) -> $t {
                value as $t
            }

            fn to_usize(&self) -> usize {
                *self as usize
            }

            fn to_i32(&self) -> i32 {
                *self as i32
            }

            fn to_f64(&self) -> f64 {
                *self as f64
            }

            fn is_inf(&self) -> bool {
                false
            }

            fn max(&self) -> Self {
                <$t>::MAX
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
                !self.clone()
            }

            fn left_shift(&self, other: &Self) -> Self {
                self << other
            }

            fn right_shift(&self, other: &Self) -> Self {
                self >> other
            }

            fn binary_repr(&self) -> String {
                format!("{self:b}")
            }
        }
    };
}
impl_numeric!(isize);
impl_numeric!(i8);
impl_numeric!(i16);
impl_numeric!(i32);
impl_numeric!(i64);
impl_numeric!(usize);
impl_numeric!(u8);
impl_numeric!(u16);
impl_numeric!(u32);
impl_numeric!(u64);

macro_rules! impl_numeric_float {
    ($t:ty, $ti:ty) => {
        impl ArrayElement for $t {

            fn zero() -> Self {
                0 as $t
            }

            fn one() -> Self {
                1 as $t
            }

            fn is_nan(&self) -> bool {
                <$t>::is_nan(*self)
            }
        }

        impl Numeric for $t {

            fn rand(range: RangeInclusive<Self>) -> $t {
                let mut rng = rand::thread_rng();
                let value = rng.sample(&Uniform::from(range));
                value as $t
            }

            fn from_usize(value: usize) -> $t {
                value as $t
            }

            fn from_f64(value: f64) -> $t {
                value as $t
            }

            fn to_usize(&self) -> usize {
                *self as usize
            }

            fn to_i32(&self) -> i32 {
                *self as i32
            }

            fn to_f64(&self) -> f64 {
                *self as f64
            }

            fn is_inf(&self) -> bool {
                <$t>::is_infinite(*self)
            }

            fn max(&self) -> Self {
                <$t>::MAX
            }

            fn bitwise_and(&self, other: &Self) -> Self {
                (*self as $ti & *other as $ti) as $t
            }

            fn bitwise_or(&self, other: &Self) -> Self {
                (*self as $ti | *other as $ti) as $t
            }

            fn bitwise_xor(&self, other: &Self) -> Self {
                (*self as $ti ^ *other as $ti) as $t
            }

            fn bitwise_not(&self) -> Self {
                !(*self as $ti) as $t
            }

            fn left_shift(&self, other: &Self) -> Self {
                ((*self as $ti) << (*other as $ti)) as $t
            }

            fn right_shift(&self, other: &Self) -> Self {
                ((*self as $ti) >> (*other as $ti)) as $t
            }

            fn binary_repr(&self) -> String {
                format!("{:b}", *self as $ti)
            }
        }
    };
}

impl_numeric_float!(f32, i64);
impl_numeric_float!(f64, i128);
