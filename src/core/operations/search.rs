use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};

/// `ArrayTrait` - Array Search functions
pub trait ArraySearch<T: ArrayElement> where Self: Sized + Clone {

    /// Returns the indices of the maximum values along an axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to search. if None, array is flattened
    /// * `keepdims` - if true, the result will broadcast correctly against the input
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(i32, [[10, 11, 12], [13, 14, 15]]);
    /// assert_eq!(array!(usize, [5]), arr.argmax(None, None));
    /// let arr = array!(f64, [[f64::NAN, 4.], [2., 3.]]);
    /// assert_eq!(array!(usize, [0]), arr.argmax(None, None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn argmax(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError>;

    /// Returns the indices of the minimum values along an axis.
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to search. if None, array is flattened
    /// * `keepdims` - if true, the result will broadcast correctly against the input
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(i32, [[10, 11, 12], [13, 14, 15]]);
    /// assert_eq!(array!(usize, [0]), arr.argmin(None, None));
    /// assert_eq!(array!(usize, [0, 0, 0]), arr.argmin(Some(0), None));
    /// assert_eq!(array!(usize, [0, 0]), arr.argmin(Some(1), None));
    /// let arr = array!(f64, [[f64::NAN, 4.], [2., 3.]]);
    /// assert_eq!(array!(usize, [0]), arr.argmin(None, None));
    /// assert_eq!(array!(usize, [0, 1]), arr.argmin(Some(0), None));
    /// assert_eq!(array!(usize, [0, 0]), arr.argmin(Some(1), None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn argmin(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError>;
}

impl <T: ArrayElement> ArraySearch<T> for Array<T> {

    fn argmax(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.argmax(None, keepdims));
            if keepdims == Some(true) { result }
            else { result.reshape(&self.get_shape()?.remove_at(axis)) }
        } else {
            if self.is_empty()? { return Err(ArrayError::ParameterError { param: "`array`", message: "cannot be empty" }) }
            let result = if let Some(i) = self.get_elements()?.iter().position(ArrayElement::is_nan) { Array::single(i) } else {
                let sorted = self.sort(None, Some("quicksort"))?.get_elements()?;
                let max_pos = self.get_elements()?.iter().position(|item| item == &sorted[sorted.len() - 1]).unwrap();
                Array::single(max_pos)
            };

            if keepdims == Some(true) { result.atleast(self.ndim()?) }
            else { result }
        }
    }

    fn argmin(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.argmin(None, keepdims));
            if keepdims == Some(true) { result }
            else { result.reshape(&self.get_shape()?.remove_at(axis)) }
        } else {
            if self.is_empty()? { return Err(ArrayError::ParameterError { param: "`array`", message: "cannot be empty" }) }
            let result = if let Some(i) = self.get_elements()?.iter().position(ArrayElement::is_nan) { Array::single(i) } else {
                let sorted = self.sort(None, Some("quicksort"))?.get_elements()?;
                let max_pos = self.get_elements()?.iter().position(|item| item == &sorted[0]).unwrap();
                Array::single(max_pos)
            };

            if keepdims == Some(true) { result.atleast(self.ndim()?) }
            else { result }
        }
    }
}

impl <T: ArrayElement> ArraySearch<T> for Result<Array<T>, ArrayError> {

    fn argmax(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        self.clone()?.argmax(axis, keepdims)
    }

    fn argmin(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        self.clone()?.argmin(axis, keepdims)
    }
}
