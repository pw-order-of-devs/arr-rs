use crate::arrays::Array;
use crate::ext::vec_ext::VecRemoveAt;
use crate::traits::{
    errors::ArrayError,
    meta::ArrayMeta,
    types::ArrayElement,
    validators::{
        validate_axis::ValidateAxis,
        validate_has_error::ValidateHasError,
    },
};

pub(crate) trait ValidateShape {

    fn is_broadcastable(&self, other: &[usize]) -> Result<(), ArrayError>;
    fn matches_values_len<T>(&self, other: &[T]) -> Result<(), ArrayError>;
    fn matches_shape(&self, other: &[usize]) -> Result<(), ArrayError>;
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
        if self.len() != other.len() {
            Err(ArrayError::ShapesMustMatch { shape_1: self.clone(), shape_2: other.to_vec() })
        } else {
            Ok(())
        }
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
