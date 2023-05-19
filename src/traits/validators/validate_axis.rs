use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::validate_has_error::ValidateHasError,
};

pub(crate) trait ValidateAxis {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError>;
    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError>;
}

impl <N: Numeric> ValidateAxis for Array<N> {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError> {
        if axis >= self.ndim()? {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }

    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError> {
        if axis.is_none() {
            Ok(())
        } else if axis.unwrap() >= self.ndim()? {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }
}

impl <N: Numeric> ValidateAxis for Vec<Array<N>> {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError> {
        self.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
        if self.iter().any(|a| axis >= a.ndim().unwrap()) {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }

    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError> {
        if axis.is_none() {
            return Ok(())
        }

        self.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
        if self.iter().any(|a| axis.unwrap() >= a.ndim().unwrap()) {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }
}
