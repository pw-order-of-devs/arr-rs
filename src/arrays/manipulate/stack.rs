use crate::arrays::Array;
use crate::ext::vec_ext::{VecInsertAt, VecRemoveAt};
use crate::traits::{
    create::ArrayCreate,
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        stack::ArrayStack,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::{
        validate_axis::ValidateAxis,
        validate_dimension::ValidateDimension,
        validate_has_error::ValidateHasError,
        validate_shape::ValidateShapeConcat,
    },
};

impl <N: Numeric> ArrayStack<N> for Array<N> {

    fn concatenate(arrs: Vec<Self>, axis: Option<usize>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Ok(Self::empty()) }
        else {
            if let Some(axis) = axis { arrs.validate_stack_shapes(axis, axis)?; }

            let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
            let result = arrs.remove_at(0).into_iter()
                .fold(initial, |a, b| a.append(&b, axis).unwrap());
            Ok(result)
        }
    }

    fn stack(arrs: Vec<Self>, axis: Option<usize>) -> Result<Self, ArrayError> {
        arrs.axis_opt_in_bounds(axis)?;
        if arrs.is_empty() { Ok(Self::empty()) }
        else if (0..arrs.len() - 1).any(|i| arrs[i].get_shape() != arrs[i + 1].get_shape()) {
            Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same shape", })
        } else {
            let axis = axis.unwrap_or(0);
            let new_shape = arrs[0].get_shape().insert_at(axis, arrs.len());

            let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
            arrs.remove_at(0).into_iter()
                .fold(initial, |a, b| a.append(&b, Some(axis)).unwrap())
                .reshape(new_shape)
        }
    }

    fn vstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Ok(Self::empty()) }
        else {
            arrs.validate_stack_shapes(0, 0)?;

            let mut new_shape = arrs[0].get_shape();
            if new_shape.len() == 1 { new_shape.insert_at(0, arrs.len()); }
            else { new_shape[0] = arrs.iter().fold(0, |a, b| a + b.shape[0]); }

            match Self::concatenate(arrs, Some(0)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn hstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Ok(Self::empty()) }
        else if arrs.iter().all(|arr| arr.ndim() == 1) {
            Self::concatenate(arrs, Some(0))
        } else {
            let arrs = arrs.iter().map(|arr| arr.atleast(2)).collect::<Vec<Result<Self, _>>>();
            arrs.has_error()?;

            let arrs = arrs.into_iter().map(|a| a.unwrap()).collect::<Vec<Self<>>>();
            arrs.validate_stack_shapes(1, 0)?;

            let mut new_shape = arrs[0].get_shape();
            new_shape[1] = arrs.iter().fold(0, |a, b| a + b.shape[1]);

            match Self::concatenate(arrs, Some(1)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn dstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Ok(Self::empty()) }
        else {
            let arrs = arrs.iter().map(|arr| arr.atleast(3)).collect::<Vec<Result<Self, _>>>();
            arrs.has_error()?;

            let arrs = arrs.into_iter().map(|a| a.unwrap()).collect::<Vec<Self<>>>();
            arrs.validate_stack_shapes(2, 0)?;

            let mut new_shape = arrs[0].get_shape();
            new_shape[2] = arrs.iter().fold(0, |a, b| a + b.shape[2]);

            match Self::concatenate(arrs, Some(2)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn column_stack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Ok(Self::empty()) }
        else {
            let (num_rows, mut total_cols) = (arrs[0].shape[0], 0);
            arrs.is_dim_supported(&[1, 2])?;
            if arrs.iter().any(|array| array.shape[0] != num_rows) {
                return Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same first dimension", });
            }

            arrs.iter().for_each(|array| {
                if array.ndim() == 1 { total_cols += 1; }
                else { total_cols += array.shape[1]; }
            });

            let (mut new_elements, mut new_col_idx) = (vec![N::ZERO; num_rows * total_cols], 0);
            arrs.iter().for_each(|array| {
                let array_cols = if array.ndim() == 1 { 1 } else { array.shape[1] };
                (0 .. num_rows).for_each(|row| {
                    (0..array_cols).for_each(|col| {
                        let src_idx = row * array_cols + col;
                        let dst_idx = row * total_cols + new_col_idx + col;
                        new_elements[dst_idx] = array.elements[src_idx];
                    })
                });
                new_col_idx += array_cols;
            });

            Self::new(new_elements, vec![num_rows, total_cols])
        }
    }

    fn row_stack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        Self::vstack(arrs)
    }
}
