use std::fmt::Debug;
use std::ops::{Range, RangeInclusive};
use crate::traits::errors::ArrayError;

pub(crate) trait ValidateEqual {

    fn is_equal(&self, other: &Self) -> Result<(), ArrayError>;
}

impl <T: PartialEq + Debug> ValidateEqual for T {

    fn is_equal(&self, other: &Self) -> Result<(), ArrayError> {
        if self == other {
            Ok(())
        } else {
            Err(ArrayError::MustBeEqual { value1: format!("{self:?}"), value2: format!("{other:?}") })
        }
    }
}

pub(crate) trait ValidateBounds<T> {

    fn is_in_bounds(&self, range: Range<T>) -> Result<(), ArrayError>;
    fn is_in_bounds_inclusive(&self, range: RangeInclusive<T>) -> Result<(), ArrayError>;
}
