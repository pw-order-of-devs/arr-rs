use std::ops::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Not,
};
use crate::traits::types::ArrayElement;

use crate::traits::types::numeric::Numeric;

/// Signed Numeric type for array
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

impl BoolNumeric for bool {}
