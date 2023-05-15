use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Stack functions
pub trait ArrayStack<N: Numeric> where Self: Sized + Clone {

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
    fn concatenate(arrs: Vec<Array<N>>, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    fn stack(arrs: Vec<Array<N>>, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    fn vstack(arrs: Vec<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    fn hstack(arrs: Vec<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    fn dstack(arrs: Vec<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    fn column_stack(arrs: Vec<Array<N>>) -> Result<Array<N>, ArrayError>;

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
    fn row_stack(arrs: Vec<Array<N>>) -> Result<Array<N>, ArrayError>;
}
