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
}
