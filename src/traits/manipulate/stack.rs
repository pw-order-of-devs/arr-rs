use crate::traits::types::numeric::Numeric;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([1, 2, 3, 4, 5, 6]);
    /// assert_eq!(expected, Array::<i32>::concatenate(vec![arr, other], None));
    ///
    /// let arr: Array<i32> = array!([[1, 2], [3, 4]]);
    /// let other: Array<i32> = array!([[5, 6]]);
    /// let expected: Array<i32> = array!([[1, 2], [3, 4], [5, 6]]);
    /// assert_eq!(expected, Array::<i32>::concatenate(vec![arr, other], Some(0)));
    /// ```
    fn concatenate(arrs: Vec<Self>, axis: Option<usize>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(expected, Array::<i32>::stack(vec![arr, other], None));
    ///
    /// let arr: Array<i32> = array!([[1, 2], [3, 4]]);
    /// let other: Array<i32> = array!([[5, 6], [7, 8]]);
    /// let expected: Array<i32> = array!([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);
    /// assert_eq!(expected, Array::<i32>::stack(vec![arr, other], Some(0)));
    /// ```
    fn stack(arrs: Vec<Self>, axis: Option<usize>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(expected, Array::<i32>::vstack(vec![arr, other]));
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]);
    /// let other: Array<i32> = array!([[4], [5], [6]]);
    /// let expected: Array<i32> = array!([[1], [2], [3], [4], [5], [6]]);
    /// assert_eq!(expected, Array::<i32>::vstack(vec![arr, other]));
    /// ```
    fn vstack(arrs: Vec<Self>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([1, 2, 3, 4, 5, 6]);
    /// assert_eq!(expected, Array::<i32>::hstack(vec![arr, other]));
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]);
    /// let other: Array<i32> = array!([[4], [5], [6]]);
    /// let expected: Array<i32> = array!([[1, 4], [2, 5], [3, 6]]);
    /// assert_eq!(expected, Array::<i32>::hstack(vec![arr, other]));
    /// ```
    fn hstack(arrs: Vec<Self>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([[[1, 4], [2, 5], [3, 6]]]);
    /// assert_eq!(expected, Array::<i32>::dstack(vec![arr, other]));
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]);
    /// let other: Array<i32> = array!([[4], [5], [6]]);
    /// let expected: Array<i32> = array!([[[1, 4]], [[2, 5]], [[3, 6]]]);
    /// assert_eq!(expected, Array::<i32>::dstack(vec![arr, other]));
    /// ```
    fn dstack(arrs: Vec<Self>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([[1, 4], [2, 5], [3, 6]]);
    /// assert_eq!(expected, Array::<i32>::column_stack(vec![arr, other]));
    /// ```
    fn column_stack(arrs: Vec<Self>) -> Self;

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
    /// let arr: Array<i32> = array!([1, 2, 3]);
    /// let other: Array<i32> = array!([4, 5, 6]);
    /// let expected: Array<i32> = array!([[1, 2, 3], [4, 5, 6]]);
    /// assert_eq!(expected, Array::<i32>::row_stack(vec![arr, other]));
    ///
    /// let arr: Array<i32> = array!([[1], [2], [3]]);
    /// let other: Array<i32> = array!([[4], [5], [6]]);
    /// let expected: Array<i32> = array!([[1], [2], [3], [4], [5], [6]]);
    /// assert_eq!(expected, Array::<i32>::row_stack(vec![arr, other]));
    /// ```
    fn row_stack(arrs: Vec<Self>) -> Self;
}
