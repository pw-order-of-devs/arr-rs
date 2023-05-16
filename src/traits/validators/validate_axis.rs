use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    meta::ArrayMeta,
    types::numeric::Numeric,
};

pub(crate) trait ValidateAxis {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError>;
    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError>;
}

impl <N: Numeric> ValidateAxis for Array<N> {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError> {
        if axis >= self.ndim() {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }

    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError> {
        if axis.is_none() {
            Ok(())
        } else if axis.unwrap() >= self.ndim() {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }
}

impl <N: Numeric> ValidateAxis for Vec<Array<N>> {

    fn axis_in_bounds(&self, axis: usize) -> Result<(), ArrayError> {
        if self.iter().any(|a| axis >= a.ndim()) {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }

    fn axis_opt_in_bounds(&self, axis: Option<usize>) -> Result<(), ArrayError> {
        if axis.is_none() {
            Ok(())
        } else if self.iter().any(|a| axis.unwrap() >= a.ndim()) {
            Err(ArrayError::AxisOutOfBounds)
        } else {
            Ok(())
        }
    }
}
