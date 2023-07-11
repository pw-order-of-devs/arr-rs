use std::cmp::Ordering;

use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::{
        iter_ext::IterSorted,
        vec_ext::{VecInsertAt, VecRemoveAt},
    },
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
    /// let arr = arr.insert(vec![1], &Array::single(1.).unwrap(), None);
    /// assert_eq!(array!([1., 1., 2., 3., 4.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.insert(vec![1, 3], &Array::single(1.).unwrap(), None);
    /// assert_eq!(array!([1., 1., 2., 3., 1., 4.]), arr);
    /// ```
    fn insert(&self, indices: Vec<usize>, values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

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
    /// let arr = arr.delete(vec![1], None);
    /// assert_eq!(array!([1., 3., 4.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.delete(vec![2, 3], None);
    /// assert_eq!(array!([1., 2.]), arr);
    /// ```
    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

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
    /// let arr = arr.reshape(vec![2, 2]);
    /// assert_eq!(array!([[1, 2], [3, 4]]), arr);
    /// ```
    fn reshape(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError>;

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
    /// let array = arr.resize(vec![2, 2]);
    /// assert_eq!(array!([[1, 2], [3, 4]]), array);
    /// let array = arr.resize(vec![4]);
    /// assert_eq!(array!([1, 2, 3, 4]), array);
    /// let array = arr.resize(vec![8]);
    /// assert_eq!(array!([1, 2, 3, 4, 1, 2, 3, 4]), array);
    /// ```
    fn resize(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError>;


    /// Find the unique elements of an array,
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
    /// let arr: Array<i32> = Array::new(vec![1, 1, 2, 3, 3, 4], vec![6]).unwrap();
    /// assert_eq!(array!([1, 2, 3, 4]), arr.unique(None));
    /// let arr: Array<i32> = Array::new(vec![1, 2, 3, 2, 1], vec![5]).unwrap();
    /// assert_eq!(array!([1, 2, 3]), arr.unique(None));
    /// ```
    fn unique(&self, axis: Option<usize>) -> Result<Array<T>, ArrayError>;

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
}

impl <T: ArrayElement> ArrayManipulate<T> for Array<T> {

    fn insert(&self, indices: Vec<usize>, values: &Self, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if indices.len() > 1 { values.is_broadcastable(&[indices.len()])?; }
        if axis.is_some() {
            self.axis_in_bounds(axis.unwrap())?;
            values.is_broadcastable(&self.get_shape()?)?;
        }

        let values = if axis.unwrap_or(0) > 0 { values.ravel()? } else { values.clone() };
        let (subarrays, chunk_size, subarray_len) =
            if let Some(axis) = axis {
                let subarrays = if axis == 0 { 1 } else { self.get_shape()?[..axis].iter().product::<usize>() };
                let chunk_size = self.get_shape()?[axis + 1..].iter().product::<usize>();
                (subarrays, chunk_size, self.len()? / subarrays)
            } else { (1, 1, self.len()?) };

        let mut new_shape = self.shape.clone();
        match axis {
            Some(axis) => {
                if self.ndim() > values.ndim() { new_shape[axis] += indices.len(); }
                else { new_shape[axis] += values.shape[axis % values.ndim()?]; }
            },
            None => new_shape = vec![self.len()? + std::cmp::max(values.len()?, indices.len())],
        }

        let axis_some_cond = axis.is_some() && !(indices.len() == 1 || values.len()? == 1 || values.len()? == chunk_size * subarrays);
        let axis_none_cond = axis.is_none() && !(indices.len() == 1 || values.len()? == 1 || indices.len() == values.len()?);
        if axis_none_cond || axis_some_cond { return Err(ArrayError::ParameterError { param: "values|indices", message: "don't match for insert", }) }

        let indices =
            if let Some(axis) = axis {
                let mut chunk = self.get_shape()?;
                chunk.remove(axis);
                let repeat_count =
                    if values.ndim()? > 1 && axis != 0 { values.get_shape()?[0] }
                    else if values.len()? == 1 { 1 }
                    else { values.len()? / chunk.iter().product::<usize>() };
                (0 .. subarrays).flat_map(|i| indices.iter()
                    .flat_map(|&x| std::iter::repeat(x).take(chunk_size * repeat_count))
                    .map(move |j| j * chunk_size + subarray_len * i)
                ).collect::<Vec<usize>>()
            }
            else if indices.len() == 1 { vec![indices[0]; values.len()?] }
            else { indices };

        let values =
            if axis.is_some() { Ok(values.clone().into_iter().cycle().take(values.len()? * indices.len()).collect()) }
            else if values.len()? == 1 { values.broadcast_to(vec![indices.len()]) }
            else { Ok(values) }?;

        let mut new_elements = self.elements.clone();
        indices.iter().sorted().rev()
            .zip(&values.get_elements()?.iter().cloned().rev().collect::<Vec<T>>())
            .for_each(|(&i, e)| new_elements.insert(i, e.clone()));

        Self::new(new_elements, new_shape)
    }

    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if let Some(axis) = axis {
            self.axis_in_bounds(axis)?;
            let mut sorted_indices = indices;
            sorted_indices.sort();
            sorted_indices.dedup();

            let new_shape = self.shape.iter()
                .enumerate()
                .map(|(i, &dim)| if i == axis { dim.saturating_sub(sorted_indices.len()) } else { dim })
                .collect::<Vec<usize>>();

            if self.elements.iter()
                .enumerate()
                .any(|(idx, _)| self.index_to_coord(idx).is_err()) {
                return Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", });
            }

            let new_elements = self.elements.iter()
                .enumerate()
                .filter_map(|(idx, elem)| {
                    let current_coords = self.index_to_coord(idx).unwrap();
                    if !sorted_indices.iter().any(|&i| i == current_coords[axis]) { Some(elem.clone()) }
                    else { None }
                }).collect::<Vec<T>>();

            Self::new(new_elements, new_shape)
        } else {
            let mut new_elements = self.get_elements()?;
            if indices.iter().any(|&i| i >= new_elements.len()) { return Err(ArrayError::OutOfBounds { value: "index", }) }
            indices.iter().rev().for_each(|&i| { new_elements.remove(i); });

            Self::flat(new_elements)
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

            let subarrays = if axis == 0 { 1 } else { self.get_shape()?[..axis].iter().product::<usize>() };
            let subarray_len = self.get_shape()?.iter().product::<usize>() / subarrays;
            let values_len = values.len()?;
            let indices = (0 .. subarrays).flat_map(|i| vec![subarray_len].iter().cycle()
                .take(values_len / subarrays)
                .map(|e| e + i * subarray_len)
                .collect::<Vec<usize>>()
            ).collect::<Vec<usize>>();

            let mut new_shape = self.get_shape()?;
            new_shape[axis] += values.get_shape()?[axis];
            let mut new_elements = self.get_elements()?;

            indices.iter().rev()
                .zip(&values.get_elements()?.iter().rev().collect::<Vec<&T>>())
                .for_each(|(&i, & e)| new_elements.insert(i, e.clone()));
            Self::new(new_elements, new_shape)
        } else {
            let mut elements = self.get_elements()?;
            elements.append(&mut values.get_elements()?);
            Self::flat(elements)
        }
    }

    fn reshape(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        shape.matches_values_len(&self.get_elements()?)?;
        Self::new(self.elements.clone(), shape)
    }

    fn resize(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        self.get_elements()?.into_iter().cycle()
            .take(shape.iter().product::<usize>())
            .collect::<Self>()
            .reshape(shape)
    }

    fn unique(&self, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if let Some(axis) = axis {
            let split = self.split(self.shape[axis], Some(axis))?;
            let mut parts = split.into_iter()
                .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .collect::<Vec<Self>>();
            parts.dedup();
            let flat_parts = parts.clone().into_iter().flatten().collect::<Vec<T>>();
            if flat_parts.len() == self.len()? { Ok(self.clone()) }
            else {
                let new_shape = self.get_shape()?
                    .remove_at(axis)
                    .insert_at(axis, parts.len());
                let result = Array::new(flat_parts, new_shape)?;
                if !(axis > 0 && axis < self.ndim()? - 1) { Ok(result) }
                else { result.rollaxis(axis as isize, None) }
            }
        } else {
            let mut new_elements = self.get_elements()?.into_iter()
                .sorted_by(|a, b| a.clone().partial_cmp(b).unwrap_or(Ordering::Equal))
                .collect::<Vec<T>>();
            new_elements.dedup();
            Array::flat(new_elements)
        }
    }

    fn ravel(&self) -> Result<Array<T>, ArrayError> {
        Self::flat(self.elements.clone())
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

        let new_elements = self.get_elements()?
            .into_iter().rev()
            .skip_while(|e| e.clone() == T::zero())
            .collect::<Vec<_>>()
            .into_iter().rev()
            .skip_while(|e| e.clone() == T::zero())
            .collect::<Vec<_>>();

        Self::flat(new_elements)
    }
}

impl <T: ArrayElement> ArrayManipulate<T> for Result<Array<T>, ArrayError> {

    fn insert(&self, indices: Vec<usize>, values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.insert(indices, values, axis)
    }

    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.delete(indices, axis)
    }

    fn append(&self, values: &Array<T>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.append(values, axis)
    }

    fn reshape(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.reshape(shape)
    }

    fn resize(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.resize(shape)
    }

    fn unique(&self, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
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
}

impl <T: ArrayElement> Array<T> {

    fn atleast_1d(&self) -> Result<Array<T>, ArrayError> {
        if !self.ndim()? >= 1 { Ok(self.clone()) }
        else { self.reshape(vec![1]) }
    }

    fn atleast_2d(&self) -> Result<Array<T>, ArrayError> {
        if self.ndim()? >= 2 { Ok(self.clone()) }
        else {
            match self.ndim()? {
                0 => self.reshape(vec![1, 1]),
                1 => self.reshape(vec![1, self.get_shape()?[0]]),
                _ => self.reshape(vec![self.get_shape()?[0], 1]),
            }
        }
    }

    fn atleast_3d(&self) -> Result<Array<T>, ArrayError> {
        if self.ndim()? >= 3 { Ok(self.clone()) }
        else {
            match self.ndim()? {
                0 => self.reshape(vec![1, 1, 1]),
                1 => self.reshape(vec![1, self.get_shape()?[0], 1]),
                2 => self.reshape(vec![self.get_shape()?[0], self.get_shape()?[1], 1]),
                _ => Ok(self.clone()),
            }
        }
    }
}

impl <T: ArrayElement> Array<T> {

    pub(crate) fn normalize_axis(axis: isize, ndim: usize) -> usize {
        if axis < 0 { (axis + ndim as isize) as usize }
        else { axis as usize }
    }
}
