use std::ops::{
    BitAnd, BitAndAssign,
    BitOr, BitOrAssign,
    BitXor, BitXorAssign,
    Not,
};

use crate::traits::types::numeric::Numeric;

/// Signed Numeric type for array
pub trait BoolNumeric: Numeric + Not +
BitAnd + BitAndAssign +
BitOr + BitOrAssign +
BitXor + BitXorAssign {}

impl BoolNumeric for bool {}
