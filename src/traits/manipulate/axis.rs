use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Axis functions
pub trait ArrayAxis<N: Numeric> where Array<N>: Sized + Clone {

    /// Returns an array with axes transposed
    ///
    /// # Arguments
    ///
    /// * `axes` - if defined, it's a list of axes to be included in transposition
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// assert_eq!(array!([[1, 5], [2, 6], [3, 7], [4, 8]]), arr.transpose(None));
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(array!([[1, 3, 5, 7], [2, 4, 6, 8]]), arr.transpose(None));
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(array!([[1, 3, 5, 7], [2, 4, 6, 8]]), arr.transpose(Some(vec![1, 0])));
    /// ```
    fn transpose(&self, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError>;

    /// Move axes of an array to new positions
    ///
    /// # Arguments
    ///
    /// * `source` - original positions of the axes to move. must be unique
    /// * `destination` - destination positions for each of the original axes. must be unique
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![4, 5, 3], arr.moveaxis(vec![0], vec![2]).get_shape().unwrap());
    /// assert_eq!(vec![5, 3, 4], arr.moveaxis(vec![2], vec![0]).get_shape().unwrap());
    /// ```
    fn moveaxis(&self, source: Vec<isize>, destination: Vec<isize>) -> Result<Array<N>, ArrayError>;

    /// Roll the specified axis backwards, until it lies in a given position
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis to be rolled
    /// * `start` - start position. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![4, 3, 5], arr.rollaxis(1, None).get_shape().unwrap());
    /// assert_eq!(vec![3, 5, 4], arr.rollaxis(2, Some(1)).get_shape().unwrap());
    /// ```
    fn rollaxis(&self, axis: isize, start: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Interchange two axes of an array
    ///
    /// # Arguments
    ///
    /// * `axis_1` - first axis
    /// * `axis_1` - second axis
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![5, 4, 3], arr.swapaxes(0, 2).get_shape().unwrap());
    /// assert_eq!(vec![3, 5, 4], arr.swapaxes(2, 1).get_shape().unwrap());
    /// ```
    fn swapaxes(&self, axis: isize, start: isize) -> Result<Array<N>, ArrayError>;

    /// Expand the shape of an array
    ///
    /// # Arguments
    ///
    /// * `axes` - position in the expanded axes where the new axis (or axes) is placed
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![3, 4, 5]);
    /// assert_eq!(vec![1, 3, 4, 5], arr.expand_dims(vec![0]).get_shape().unwrap());
    /// assert_eq!(vec![3, 1, 4, 1, 5], arr.expand_dims(vec![1, 3]).get_shape().unwrap());
    /// ```
    fn expand_dims(&self, axes: Vec<isize>) -> Result<Array<N>, ArrayError>;

    /// Remove axes of length one from array
    ///
    /// # Arguments
    ///
    /// * `axes` - position of the 10-sized axes to remove. if None, all such axes will be removed
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::zeros(vec![1, 3, 1, 4, 5]);
    /// assert_eq!(vec![3, 4, 5], arr.squeeze(None).get_shape().unwrap());
    /// assert_eq!(vec![3, 1, 4, 5], arr.squeeze(Some(vec![0])).get_shape().unwrap());
    /// assert_eq!(vec![1, 3, 4, 5], arr.squeeze(Some(vec![2])).get_shape().unwrap());
    /// ```
    fn squeeze(&self, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError>;
}
