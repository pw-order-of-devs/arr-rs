use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    meta::ArrayMeta,
    types::ArrayElement,
    validators::validate_has_error::ValidateHasError,
};

pub(crate) trait ValidateDimension {

    fn is_dim_supported(&self, supported: &[usize]) -> Result<(), ArrayError>;
    fn is_dim_unsupported(&self, unsupported: &[usize]) -> Result<(), ArrayError>;
}

impl ValidateDimension for usize {

    fn is_dim_supported(&self, supported: &[usize]) -> Result<(), ArrayError> {
        if !supported.contains(self) {
            Err(ArrayError::UnsupportedDimension { supported: supported.to_vec() })
        } else {
            Ok(())
        }
    }

    fn is_dim_unsupported(&self, unsupported: &[usize]) -> Result<(), ArrayError> {
        if unsupported.contains(self) {
            Err(ArrayError::UnsupportedDimension { supported: unsupported.to_vec() })
        } else {
            Ok(())
        }
    }
}

impl <T: ArrayElement> ValidateDimension for Array<T> {

    fn is_dim_supported(&self, supported: &[usize]) -> Result<(), ArrayError> {
        if !supported.contains(&self.ndim()?) {
            Err(ArrayError::UnsupportedDimension { supported: supported.to_vec() })
        } else {
            Ok(())
        }
    }

    fn is_dim_unsupported(&self, unsupported: &[usize]) -> Result<(), ArrayError> {
        if unsupported.contains(&self.ndim()?) {
            Err(ArrayError::UnsupportedDimension { supported: unsupported.to_vec() })
        } else {
            Ok(())
        }
    }
}

impl <T: ArrayElement> ValidateDimension for Vec<Array<T>> {

    fn is_dim_supported(&self, unsupported: &[usize]) -> Result<(), ArrayError> {
        self.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
        if self.iter().any(|a| !unsupported.contains(&a.ndim().unwrap())) {
            Err(ArrayError::UnsupportedDimension { supported: unsupported.to_vec() })
        } else {
            Ok(())
        }
    }

    fn is_dim_unsupported(&self, unsupported: &[usize]) -> Result<(), ArrayError> {
        self.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
        if self.iter().any(|a| unsupported.contains(&a.ndim().unwrap())) {
            Err(ArrayError::UnsupportedDimension { supported: unsupported.to_vec() })
        } else {
            Ok(())
        }
    }
}
