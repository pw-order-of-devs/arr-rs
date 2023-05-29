use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Reordering functions
pub trait ArrayReorder<N: Numeric> where Self: Sized + Clone {

    /// Reverse the order of elements in an array along the given axis
    ///
    /// # Arguments
    ///
    /// * `axes` - axes along which to flip over. if None, will flip over all of the axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[1, 0], [3, 2]], [[5, 4], [7, 6]]]), arr.flip(Some(vec![2])));
    /// ```
    fn flip(&self, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError>;

    /// Reverse the order of elements along axis 0 (up/down)
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]]), arr.flipud());
    /// ```
    fn flipud(&self) -> Result<Array<N>, ArrayError>;

    /// Reverse the order of elements along axis 1 (left/right)
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[2, 3], [0, 1]], [[6, 7], [4, 5]]]), arr.fliplr());
    /// ```
    fn fliplr(&self) -> Result<Array<N>, ArrayError>;

    /// Roll array elements along a given axis
    ///
    /// # Arguments
    ///
    /// * `shift` - number of places by which elements are shifted.
    /// if a tuple, then axis must be a tuple of the same size, and each of the given axes is shifted by the corresponding number.
    /// if an int while axis is a tuple of ints, then the same value is used for all given axes.
    /// * `axes` - axes along which to roll over. if None, will flip over all of the axes.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// assert_eq!(array!([[[4, 5], [6, 7]], [[0, 1], [2, 3]]]), arr.roll(vec![1], Some(vec![0])));
    /// ```
    fn roll(&self, shift: Vec<isize>, axes: Option<Vec<isize>>) -> Result<Array<N>, ArrayError>;

    /// Rotate an array by 90 degrees in the plane specified by axes.
    /// Rotation direction is from the first towards the second axis.
    ///
    /// # Arguments
    ///
    /// * `k` - number of times the array is rotated by 90 degrees.
    /// * `axes` - the array is rotated in the plane defined by the axes. axes must be different.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 4]);
    /// assert_eq!(array!([[3, 7], [2, 6], [1, 5], [0, 4]]), arr.rot90(1, vec![0, 1]));
    /// ```
    fn rot90(&self, k: usize, axes: Vec<isize>) -> Result<Array<N>, ArrayError>;
}
