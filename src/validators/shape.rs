use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    validators::prelude::*,
};
pub(crate) trait ValidateShape {

    fn is_broadcastable(&self, other: &[usize]) -> Result<(), ArrayError>;
    fn matches_values_len<T>(&self, other: &[T]) -> Result<(), ArrayError>;
    fn matches_shape(&self, other: &[usize]) -> Result<(), ArrayError>;
    fn shapes_align(&self, i: usize, other: &[usize], j: usize) -> Result<(), ArrayError>;
    fn is_square(&self) -> Result<(), ArrayError>;
}

impl ValidateShape for Vec<usize> {

    fn is_broadcastable(&self, other: &[usize]) -> Result<(), ArrayError> {
        if self.iter()
            .zip(other.iter())
            .take(self.len().max(other.len()))
            .rev()
            .any(|(&dim1, &dim2)| dim1 != dim2 && dim1 != 1 && dim2 != 1 || dim1 == 0 || dim2 == 0) {
            Err(ArrayError::BroadcastShapeMismatch)
        } else {
            Ok(())
        }
    }

    fn matches_values_len<T>(&self, other: &[T]) -> Result<(), ArrayError> {
        if self.iter().product::<usize>() != other.len() {
            Err(ArrayError::ShapeMustMatchValuesLength)
        } else {
            Ok(())
        }
    }

    fn matches_shape(&self, other: &[usize]) -> Result<(), ArrayError> {
        if self != other {
            Err(ArrayError::ShapesMustMatch { shape_1: self.clone(), shape_2: other.to_vec() })
        } else {
            Ok(())
        }
    }

    fn shapes_align(&self, i: usize, other: &[usize], j: usize) -> Result<(), ArrayError> {
        if self[i] != other[j] {
            Err(ArrayError::ParameterError { param: "`shapes`", message: "are not aligned" })
        } else {
            Ok(())
        }
    }

    fn is_square(&self) -> Result<(), ArrayError> {
        self.len().is_at_least(&2)?;
        let last = self.len() - 1;
        let last_prev = self.len() - 2;
        self[last].is_at_least(&2)?;
        self[last_prev].is_at_least(&2)?;
        self[last].is_equal(&self[last_prev])?;
        Ok(())
    }
}

impl <T: ArrayElement> ValidateShape for Array<T> {

    fn is_broadcastable(&self, other: &[usize]) -> Result<(), ArrayError> {
        self.get_shape()?.is_broadcastable(other)
    }

    fn matches_values_len<S>(&self, other: &[S]) -> Result<(), ArrayError> {
        self.get_shape()?.matches_values_len(other)
    }

    fn matches_shape(&self, other: &[usize]) -> Result<(), ArrayError> {
        self.get_shape()?.matches_shape(other)
    }

    fn shapes_align(&self, i: usize, other: &[usize], j: usize) -> Result<(), ArrayError> {
        self.get_shape()?.shapes_align(i, other, j)
    }

    fn is_square(&self) -> Result<(), ArrayError> {
        self.is_dim_unsupported(&[0, 1])?;
        let shape = self.get_shape()?;
        shape[0].is_at_least(&2)?;
        shape[1].is_at_least(&2)?;
        shape[0].is_equal(&shape[1])?;
        Ok(())
    }
}

pub(crate) trait ValidateShapeConcat {

    fn validate_stack_shapes(&self, axis: usize, remove_at: usize) -> Result<(), ArrayError>;
}

impl <T: ArrayElement> ValidateShapeConcat for Vec<Array<T>> {

    fn validate_stack_shapes(&self, axis: usize, remove_at: usize) -> Result<(), ArrayError> {
        self.axis_in_bounds(axis)?;
        self.iter().map(|a| a.get_shape()).collect::<Vec<Result<Vec<usize>, ArrayError>>>().has_error()?;
        if (0 .. self.len() - 1).any(|i| {
            let shape_1 = self[i].get_shape().unwrap().remove_at(remove_at);
            let shape_2 = self[i + 1].get_shape().unwrap().remove_at(remove_at);
            shape_1 != shape_2
        }) {
            Err(ArrayError::ConcatenateShapeMismatch)
        } else {
            Ok(())
        }
    }
}
