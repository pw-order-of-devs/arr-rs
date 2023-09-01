use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};

/// ArrayTrait - Array Count functions
pub trait ArrayCount<T: ArrayElement> where Self: Sized + Clone {

    /// Sort an array
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to count. if None, array is flattened
    /// * `keepdims` - if true, the result will broadcast correctly against the input
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_eye!(i32, 4);
    /// assert_eq!(array!(usize, [4]), arr.count_nonzero(None, None));
    ///
    /// let arr = array!(i32, [[0, 1, 7, 0], [3, 0, 2, 19]]);
    /// assert_eq!(array!(usize, [1, 1, 2, 1]), arr.count_nonzero(Some(0), None));
    /// ```
    fn count_nonzero(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError>;
}

impl <T: ArrayElement> ArrayCount<T> for Array<T> {

    fn count_nonzero(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.count_nonzero(None, keepdims));
            if let Some(true) = keepdims { result }
            else { result.reshape(&self.get_shape()?.remove_at(axis)) }
        } else {
            let filtered = self.get_elements()?.into_iter()
                .filter(|e| e != &T::zero())
                .collect::<Vec<T>>();
            let result = Array::single(filtered.len());
            if let Some(true) = keepdims { result.atleast(self.ndim()?) }
            else { result }
        }
    }
}

impl <T: ArrayElement> ArrayCount<T> for Result<Array<T>, ArrayError> {

    fn count_nonzero(&self, axis: Option<isize>, keepdims: Option<bool>) -> Result<Array<usize>, ArrayError> {
        self.clone()?.count_nonzero(axis, keepdims)
    }
}
