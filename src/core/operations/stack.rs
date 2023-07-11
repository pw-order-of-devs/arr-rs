use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::vec_ext::{VecInsertAt, VecRemoveAt},
    validators::prelude::*,
};

/// ArrayTrait - Array Stack functions
pub trait ArrayStack<T: ArrayElement> where Self: Sized + Clone {

    /// Join a sequence of arrays along an existing axis
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to concatenate
    /// * `axis` - the axis along which to concat. optional, if None - arrays are flattened
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([1, 2, 3, 4, 5, 6]).unwrap();
    /// assert_eq!(expected, Array::<i32>::concatenate(vec![arr, other], None).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1, 2], [3, 4]]).unwrap();
    /// let other: Array<i32> = array!([[5, 6]]).unwrap();
    /// let expected: Array<i32> = array!([[1, 2], [3, 4], [5, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::concatenate(vec![arr, other], Some(0)).unwrap());
    /// ```
    fn concatenate(arrs: Vec<Array<T>>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

    /// Join a sequence of arrays along a new axis
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    /// * `axis` - the axis along which to concat. optional, defaults to 0
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::stack(vec![arr, other], None).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1, 2], [3, 4]]).unwrap();
    /// let other: Array<i32> = array!([[5, 6], [7, 8]]).unwrap();
    /// let expected: Array<i32> = array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::stack(vec![arr, other], Some(0)).unwrap());
    /// ```
    fn stack(arrs: Vec<Array<T>>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

    /// Stack arrays in sequence vertically (row wise)
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::vstack(vec![arr, other]).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let other: Array<i32> = array!([[4], [5], [6]]).unwrap();
    /// let expected: Array<i32> = array!([[1], [2], [3], [4], [5], [6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::vstack(vec![arr, other]).unwrap());
    /// ```
    fn vstack(arrs: Vec<Array<T>>) -> Result<Array<T>, ArrayError>;

    /// Stack arrays in sequence horizontally (column wise)
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([1, 2, 3, 4, 5, 6]).unwrap();
    /// assert_eq!(expected, Array::<i32>::hstack(vec![arr, other]).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let other: Array<i32> = array!([[4], [5], [6]]).unwrap();
    /// let expected: Array<i32> = array!([[1, 4], [2, 5], [3, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::hstack(vec![arr, other]).unwrap());
    /// ```
    fn hstack(arrs: Vec<Array<T>>) -> Result<Array<T>, ArrayError>;

    /// Stack arrays in sequence depth wise (along third axis)
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([[[1, 4], [2, 5], [3, 6]]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::dstack(vec![arr, other]).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let other: Array<i32> = array!([[4], [5], [6]]).unwrap();
    /// let expected: Array<i32> = array!([[[1, 4]], [[2, 5]], [[3, 6]]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::dstack(vec![arr, other]).unwrap());
    /// ```
    fn dstack(arrs: Vec<Array<T>>) -> Result<Array<T>, ArrayError>;

    /// Stack 1d or 2d arrays as columns into a 2d array
    /// row_stack is an alias for vstack
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([[1, 4], [2, 5], [3, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::column_stack(vec![arr, other]).unwrap());
    /// ```
    fn column_stack(arrs: Vec<Array<T>>) -> Result<Array<T>, ArrayError>;

    /// Stack arrays in sequence vertically (row wise)
    ///
    /// # Arguments
    ///
    /// * `arrs` - arrays to stack
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3]).unwrap();
    /// let other: Array<i32> = array!([4, 5, 6]).unwrap();
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::row_stack(vec![arr, other]).unwrap());
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let other: Array<i32> = array!([[4], [5], [6]]).unwrap();
    /// let expected: Array<i32> = array!([[1], [2], [3], [4], [5], [6]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::row_stack(vec![arr, other]).unwrap());
    /// ```
    fn row_stack(arrs: Vec<Array<T>>) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayStack<T> for Array<T> {

    fn concatenate(arrs: Vec<Self>, axis: Option<usize>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Self::empty() }
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
        if arrs.is_empty() { Self::empty() }
        else if (0..arrs.len() - 1).any(|i| arrs[i].get_shape() != arrs[i + 1].get_shape()) {
            Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same shape", })
        } else {
            let axis = axis.unwrap_or(0);
            let new_shape = arrs[0].get_shape()?.insert_at(axis, arrs.len());

            let (mut arrs, initial) = (arrs.clone(), arrs[0].clone());
            arrs.remove_at(0).into_iter()
                .fold(initial, |a, b| a.append(&b, Some(axis)).unwrap())
                .reshape(new_shape)
        }
    }

    fn vstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Self::empty() }
        else {
            arrs.validate_stack_shapes(0, 0)?;

            let mut new_shape = arrs[0].get_shape()?;
            if new_shape.len() == 1 { new_shape.insert_at(0, arrs.len()); }
            else { new_shape[0] = arrs.iter().fold(0, |a, b| a + b.shape[0]); }

            match Self::concatenate(arrs, Some(0)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn hstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { return Self::empty() }
        arrs.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
        if arrs.iter().all(|arr| arr.ndim().unwrap() == 1) {
            Self::concatenate(arrs, Some(0))
        } else {
            let arrs = arrs.iter()
                .map(|arr| arr.atleast(2)).collect::<Vec<Result<Self, _>>>()
                .has_error()?.into_iter()
                .map(|a| a.unwrap())
                .collect::<Vec<Self<>>>();
            arrs.validate_stack_shapes(1, 0)?;

            let mut new_shape = arrs[0].get_shape()?;
            new_shape[1] = arrs.iter().fold(0, |a, b| a + b.shape[1]);

            match Self::concatenate(arrs, Some(1)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn dstack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Self::empty() }
        else {
            let arrs = arrs.iter()
                .map(|arr| arr.atleast(3))
                .collect::<Vec<Result<Self, _>>>()
                .has_error()?.into_iter()
                .map(|a| a.unwrap())
                .collect::<Vec<Self<>>>();
            arrs.validate_stack_shapes(2, 0)?;

            let mut new_shape = arrs[0].get_shape()?;
            new_shape[2] = arrs.iter().fold(0, |a, b| a + b.shape[2]);

            match Self::concatenate(arrs, Some(2)) {
                Ok(c) => c.reshape(new_shape),
                Err(e) => Err(e),
            }
        }
    }

    fn column_stack(arrs: Vec<Self>) -> Result<Self, ArrayError> {
        if arrs.is_empty() { Self::empty() }
        else {
            let (num_rows, mut total_cols) = (arrs[0].shape[0], 0);
            arrs.is_dim_supported(&[1, 2])?;
            if arrs.iter().any(|array| array.shape[0] != num_rows) {
                return Err(ArrayError::ParameterError { param: "arrs", message: "all input arrays must have the same first dimension", });
            }

            arrs.iter().map(|a| a.ndim()).collect::<Vec<Result<usize, ArrayError>>>().has_error()?;
            arrs.iter().for_each(|array| {
                if array.ndim().unwrap() == 1 { total_cols += 1; }
                else { total_cols += array.shape[1]; }
            });

            let (mut new_elements, mut new_col_idx) = (vec![T::zero(); num_rows * total_cols], 0);
            arrs.iter().for_each(|array| {
                let array_cols = if array.ndim().unwrap() == 1 { 1 } else { array.shape[1] };
                (0 .. num_rows).for_each(|row| {
                    (0..array_cols).for_each(|col| {
                        let src_idx = row * array_cols + col;
                        let dst_idx = row * total_cols + new_col_idx + col;
                        new_elements[dst_idx] = array.elements[src_idx].clone();
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
