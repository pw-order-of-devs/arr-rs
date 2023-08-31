use std::cmp::Ordering;

use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Manipulate functions
pub trait ArrayManipulate<T: ArrayElement> where Array<T>: Sized + Clone {

    /// Insert values along the given axis for the given indices
    ///
    /// # Arguments
    ///
    /// * `index` - indices before which values is inserted
    /// * `values` - vector representing values to insert into array
    /// * `axis` - axis along which to insert values. if None, then array is flattened first
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.insert(&[1], &Array::single(1.).unwrap(), None);
    /// assert_eq!(array!([1., 1., 2., 3., 4.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.insert(&[1, 3], &Array::single(1.).unwrap(), None);
    /// assert_eq!(array!([1., 1., 2., 3., 1., 4.]), arr);
    /// ```
    fn insert(&self, indices: &[usize], values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

    /// Delete values along the given axis
    ///
    /// # Arguments
    ///
    /// * `indices` - vector representing values to delete from array
    /// * `axis` - axis along which to find unique values. if None, then array is flattened first
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.delete(&[1], None);
    /// assert_eq!(array!([1., 3., 4.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.delete(&[2, 3], None);
    /// assert_eq!(array!([1., 2.]), arr);
    /// ```
    fn delete(&self, indices: &[usize], axis: Option<usize>) -> Result<Array<T>, ArrayError>;

    /// Append values to the end of an array
    ///
    /// # Arguments
    ///
    /// * `values` - vector representing values to append to array
    /// * `axis` - axis along which to append values. if None, then array is flattened first
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.append(&Array::single(1.).unwrap(), None);
    /// assert_eq!(array!([1., 2., 3., 4., 1.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.append(&Array::flat(vec![1., 3.]).unwrap(), None);
    /// assert_eq!(array!([1., 2., 3., 4., 1., 3.]), arr);
    /// ```
    fn append(&self, values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

    /// Reshapes an array
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// assert_eq!(array!([1, 2, 3, 4]).unwrap(), arr);
    /// let arr = arr.reshape(&[2, 2]);
    /// assert_eq!(array!([[1, 2], [3, 4]]), arr);
    /// ```
    fn reshape(&self, shape: &[usize]) -> Result<Array<T>, ArrayError>;

    /// Resizes an array,
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing new array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// assert_eq!(array!([1, 2, 3, 4]).unwrap(), arr);
    /// let array = arr.resize(&[2, 2]);
    /// assert_eq!(array!([[1, 2], [3, 4]]), array);
    /// let array = arr.resize(&[4]);
    /// assert_eq!(array!([1, 2, 3, 4]), array);
    /// let array = arr.resize(&[8]);
    /// assert_eq!(array!([1, 2, 3, 4, 1, 2, 3, 4]), array);
    /// ```
    fn resize(&self, shape: &[usize]) -> Result<Array<T>, ArrayError>;


    /// Find the unique elements of an array
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to split. optional, if None, array will be flattened
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1, 1, 2, 3, 3, 4], vec![6]).unwrap();
    /// assert_eq!(array!([1, 2, 3, 4]), arr.unique(None));
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 2, 1], vec![5]).unwrap();
    /// assert_eq!(array!([1, 2, 3]), arr.unique(None));
    /// ```
    fn unique(&self, axis: Option<isize>) -> Result<Array<T>, ArrayError>;

    /// Return a contiguous flattened array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = vec![8];
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// assert_eq!(expected, arr_1.ravel().get_shape().unwrap());
    ///
    /// let arr_2 = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(expected, arr_2.ravel().get_shape().unwrap());
    ///
    /// let arr_3 = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    /// assert_eq!(expected, arr_3.ravel().get_shape().unwrap());
    /// ```
    fn ravel(&self) -> Result<Array<T>, ArrayError>;

    /// Convert array to at least n dimension
    ///
    /// # Arguments
    ///
    /// * `n` - desired dimension
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1], vec![1]).unwrap();
    /// assert_eq!(array!([[1]]), arr.atleast(2));
    /// assert_eq!(array!([[[1]]]), arr.atleast(3));
    /// ```
    fn atleast(&self, n: usize) -> Result<Array<T>, ArrayError>;

    /// Trim the leading and/or trailing zeros from a 1D array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![0, 0, 1, 2, 3, 4, 0, 0]);
    /// assert_eq!(array!([1, 2, 3, 4]), arr.trim_zeros());
    /// ```
    fn trim_zeros(&self) -> Result<Array<T>, ArrayError>;

    /// Performs cycle().take(n), returning flattened array
    ///
    /// # Arguments
    ///
    /// n: number of elements to take
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(array!([1, 2]), arr.cycle_take(2));
    /// ```
    fn cycle_take(&self, n: usize) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayManipulate<T> for Array<T> {

    fn insert(&self, indices: &[usize], values: &Self, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if indices.iter().any(|&i| i > self.shape[axis.unwrap_or(0)]) { return Err(ArrayError::OutOfBounds { value: "index" }) }
        values.ndim()?.is_dim_supported(&(1 ..= self.ndim()?).collect::<Vec<usize>>())?;

        if let Some(axis) = axis {
            vec![indices.len()].is_broadcastable(&self.get_shape()?)?;
            let mut arrays = self.split_axis(axis)?;
            let self_rem = self.get_shape()?.remove_at(axis);
            let self_rem_len = self_rem.into_iter().product::<usize>();
            let mut values = values.to_array_ndim(self.ndim()?)?;
            let values_shape_tmp = values.get_shape()?.swap_ext(0, axis);

            for i in (0 .. self.ndim()?).collect::<Vec<usize>>().remove_at(axis).reverse_ext() {
                let self_shape_at_i = self.get_shape()?[i];
                if values_shape_tmp[i] > self_shape_at_i || self_shape_at_i % values_shape_tmp[i] != 0 {
                    return Err(ArrayError::BroadcastShapeMismatch)
                } else if values_shape_tmp[i] < self_shape_at_i {
                    values = values
                        .repeat(&[self_shape_at_i / values_shape_tmp[i]], Some(0))?
                        .to_array_ndim(self.ndim()?)?;
                }
            };

            let mut values =
                if indices.len() > 1 {
                    let values =
                        if values.len()? == self_rem_len { values.repeat(&[indices.len()], Some(0))? }
                        else { values };
                    values.moveaxis(vec![axis as isize], vec![0])
                        .ravel()
                        .split(indices.len(), None)?
                } else { vec![values] };

            for (i, v) in indices.to_vec().reverse_ext().into_iter().zip(values.reverse_ext()) {
                arrays.insert(i, v.clone())
            }

            let partial = arrays.into_iter()
                .map(|arr| arr.get_elements())
                .collect::<Vec<Result<Vec<T>, _>>>()
                .has_error()?.into_iter()
                .flat_map(|v| v.unwrap())
                .collect::<Array<T>>();
            let new_shape = self.get_shape()?
                .update_at(axis, partial.len()? / self_rem_len)
                .swap_ext(0, axis);
            let partial = partial.reshape(&new_shape)?;
            let transpose_shape = (1 .. self.ndim()? as isize).collect::<Vec<isize>>().insert_at(axis, 0);
            partial.transpose(Some(transpose_shape))
        } else {
            values.ndim()?.is_dim_supported(&[1])?;
            let (indices, values) = Array::broadcast_h2(&Array::flat(indices.to_vec())?, &values.ravel()?)?;
            let values = values.get_elements()?;
            let mut elements = self.get_elements()?;

            indices.get_elements()?.into_iter()
                .zip(&values)
                .sorted_by(|(a, _), (b, _)| a.cmp(b)).rev()
                .for_each(|(i, v)| elements.insert(i, v.clone()));
            elements.to_array()
        }
    }

    fn delete(&self, indices: &[usize], axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        let mut indices = indices.to_vec();
        indices.sort();
        indices.dedup();
        indices = indices.reverse_ext();

        if let Some(axis) = axis {
            self.apply_along_axis(axis, |arr| arr.delete(&indices, None))
        } else {
            let mut elements = self.get_elements()?;
            if indices.iter().any(|&i| i >= elements.len()) { return Err(ArrayError::OutOfBounds { value: "index" }) }
            indices.iter().for_each(|&i| { elements.remove(i); });
            elements.to_array()
        }
    }

    fn append(&self, values: &Self, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if let Some(axis) = axis {
            self.axis_in_bounds(axis)?;
            if self.ndim().is_equal(&values.ndim()).is_err() {
                return Err(ArrayError::ParameterError { param: "values", message: "input array should have the same dimension as the original one", })
            } else if self.get_shape()?.remove_at(axis).is_equal(&values.get_shape()?.remove_at(axis)).is_err() {
                return Err(ArrayError::ParameterError { param: "axis", message: "input array dimensions for the concatenation axis must match exactly", })
            }

            let mut arrays = self.split_axis(axis)?;
            let self_rem = self.get_shape()?.remove_at(axis);
            let self_rem_len = self_rem.into_iter().product::<usize>();
            let values = values.split_axis(axis)?;
            arrays.extend_from_slice(&values);
            let array = arrays.into_iter().flatten().collect::<Array<T>>();
            let new_shape = self.get_shape()?.update_at(axis, array.len()? / self_rem_len);
            let tmp_shape = new_shape.clone().swap_ext(0, axis);
            let transpose_shape = (1 .. self.ndim()? as isize).collect::<Vec<isize>>().insert_at(axis, 0);
            array.reshape(&tmp_shape).transpose(Some(transpose_shape)).reshape(&new_shape)
        } else {
            let mut elements = self.get_elements()?;
            elements.extend_from_slice(&values.get_elements()?);
            elements.to_array()
        }
    }

    fn reshape(&self, shape: &[usize]) -> Result<Array<T>, ArrayError> {
        shape.to_vec().matches_values_len(&self.get_elements()?)?;
        Array::new(self.elements.clone(), shape.to_vec())
    }

    fn resize(&self, shape: &[usize]) -> Result<Array<T>, ArrayError> {
        self.get_elements()?.into_iter().cycle()
            .take(shape.iter().product::<usize>())
            .collect::<Self>()
            .reshape(shape)
    }

    fn unique(&self, axis: Option<isize>) -> Result<Array<T>, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.unique(None))
        } else {
            let mut new_elements = self.get_elements()?.into_iter()
                .sorted_by(|a, b| a.clone().partial_cmp(b).unwrap_or(Ordering::Equal))
                .collect::<Vec<T>>();
            new_elements.dedup();
            new_elements.to_array()
        }
    }

    fn ravel(&self) -> Result<Array<T>, ArrayError> {
        self.elements.to_array()
    }

    fn atleast(&self, n: usize) -> Result<Array<T>, ArrayError> {
        match n {
            0 => Ok(self.clone()),
            1 => Self::atleast_1d(self),
            2 => Self::atleast_2d(self),
            3 => Self::atleast_3d(self),
            _ => Err(ArrayError::UnsupportedDimension { supported: vec![0, 1, 2, 3] }),
        }
    }

    fn trim_zeros(&self) -> Result<Array<T>, ArrayError> {
        self.is_dim_supported(&[1])?;

        self.get_elements()?
            .into_iter().rev()
            .skip_while(|e| e.clone() == T::zero())
            .collect::<Vec<_>>()
            .into_iter().rev()
            .skip_while(|e| e.clone() == T::zero())
            .collect::<Vec<_>>()
            .to_array()
    }

    fn cycle_take(&self, n: usize) -> Result<Array<T>, ArrayError> {
        Ok(self.into_iter().cycle().take(n).cloned().collect::<Array<T>>())
    }
}

impl <T: ArrayElement> ArrayManipulate<T> for Result<Array<T>, ArrayError> {

    fn insert(&self, indices: &[usize], values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.insert(indices, values, axis)
    }

    fn delete(&self, indices: &[usize], axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.delete(indices, axis)
    }

    fn append(&self, values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.append(values, axis)
    }

    fn reshape(&self, shape: &[usize]) -> Result<Array<T>, ArrayError> {
        self.clone()?.reshape(shape)
    }

    fn resize(&self, shape: &[usize]) -> Result<Array<T>, ArrayError> {
        self.clone()?.resize(shape)
    }

    fn unique(&self, axis: Option<isize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.unique(axis)
    }

    fn ravel(&self) -> Result<Array<T>, ArrayError> {
        self.clone()?.ravel()
    }

    fn atleast(&self, n: usize) -> Result<Array<T>, ArrayError> {
        self.clone()?.atleast(n)
    }

    fn trim_zeros(&self) -> Result<Array<T>, ArrayError> {
        self.clone()?.trim_zeros()
    }

    fn cycle_take(&self, n: usize) -> Result<Array<T>, ArrayError> {
        self.clone()?.cycle_take(n)
    }
}

impl <T: ArrayElement> Array<T> {

    fn atleast_1d(&self) -> Result<Array<T>, ArrayError> {
        if !self.ndim()? >= 1 { Ok(self.clone()) }
        else { self.reshape(&[1]) }
    }

    fn atleast_2d(&self) -> Result<Array<T>, ArrayError> {
        if self.ndim()? >= 2 { Ok(self.clone()) }
        else {
            match self.ndim()? {
                0 => self.reshape(&[1, 1]),
                1 => self.reshape(&[1, self.get_shape()?[0]]),
                _ => self.reshape(&[self.get_shape()?[0], 1]),
            }
        }
    }

    fn atleast_3d(&self) -> Result<Array<T>, ArrayError> {
        if self.ndim()? >= 3 { Ok(self.clone()) }
        else {
            match self.ndim()? {
                0 => self.reshape(&[1, 1, 1]),
                1 => self.reshape(&[1, self.get_shape()?[0], 1]),
                2 => self.reshape(&[self.get_shape()?[0], self.get_shape()?[1], 1]),
                _ => Ok(self.clone()),
            }
        }
    }
}

impl <T: ArrayElement> Array<T> {

    pub(crate) fn normalize_axis(&self, axis: isize) -> usize {
        if axis < 0 { (axis + self.ndim().unwrap() as isize) as usize }
        else { axis as usize }
    }

    pub(crate) fn normalize_axis_dim(&self, axis: isize, ndim: usize) -> usize {
        if axis < 0 { (self.ndim().unwrap() as isize + axis + ndim as isize) as usize }
        else { axis as usize }
    }
}
