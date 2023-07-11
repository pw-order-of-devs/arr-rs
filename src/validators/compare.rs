use std::fmt::Debug;

use crate::errors::ArrayError;

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
