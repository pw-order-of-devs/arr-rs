use std::fmt::Debug;

use crate::errors::ArrayError;

pub(crate) trait ValidateEqual {

    fn is_equal(&self, other: &Self) -> Result<(), ArrayError>;

    fn is_at_least(&self, other: &Self) -> Result<(), ArrayError>;

    fn is_one_of(&self, other: Vec<&Self>) -> Result<(), ArrayError>;
}

impl <T: PartialEq + PartialOrd + Debug> ValidateEqual for T {

    fn is_equal(&self, other: &Self) -> Result<(), ArrayError> {
        if self == other {
            Ok(())
        } else {
            Err(ArrayError::MustBeEqual { value1: format!("{self:?}"), value2: format!("{other:?}") })
        }
    }

    fn is_at_least(&self, other: &Self) -> Result<(), ArrayError> {
        if self >= other {
            Ok(())
        } else {
            Err(ArrayError::MustBeAtLeast { value1: format!("{self:?}"), value2: format!("{other:?}") })
        }
    }

    fn is_one_of(&self, other: Vec<&Self>) -> Result<(), ArrayError> {
        if other.contains(&self) {
            Ok(())
        } else {
            Err(ArrayError::MustBeEqual { value1: format!("{self:?}"), value2: format!("{other:?}") })
        }
    }
}
