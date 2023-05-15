/// Array Axis functions
pub mod axis;
/// Array Broadcast functions
pub mod broadcast;
/// Array Iterable functions
pub mod iter;
/// Array Split functions
pub mod split;
/// Array Stack functions
pub mod stack;

use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Manipulate functions
pub trait ArrayManipulate<N: Numeric> where Array<N>: Sized + Clone {

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
    /// let arr = arr.insert(vec![1], &Array::single(1.), None);
    /// assert_eq!(array!([1., 1., 2., 3., 4.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.insert(vec![1, 3], &Array::single(1.), None);
    /// assert_eq!(array!([1., 1., 2., 3., 1., 4.]), arr);
    /// ```
    fn insert(&self, indices: Vec<usize>, values: &Array<N>, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    /// let arr = arr.append(&Array::single(1.), None);
    /// assert_eq!(array!([1., 2., 3., 4., 1.]), arr);
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]).unwrap();
    /// let arr = arr.append(&Array::flat(vec![1., 3.]), None);
    /// assert_eq!(array!([1., 2., 3., 4., 1., 3.]), arr);
    /// ```
    fn append(&self, values: &Array<N>, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    fn reshape(&self, shape: Vec<usize>) -> Result<Array<N>, ArrayError>;

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
    fn resize(&self, shape: Vec<usize>) -> Result<Array<N>, ArrayError>;


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
    fn unique(&self, axis: Option<usize>) -> Result<Array<N>, ArrayError>;

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
    /// assert_eq!(expected, arr_1.ravel().get_shape());
    ///
    /// let arr_2 = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]).unwrap();
    /// assert_eq!(expected, arr_2.ravel().get_shape());
    ///
    /// let arr_3 = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    /// assert_eq!(expected, arr_3.ravel().get_shape());
    /// ```
    fn ravel(&self) -> Result<Array<N>, ArrayError>;

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
    fn atleast(&self, n: usize) -> Result<Array<N>, ArrayError>;

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
    fn trim_zeros(&self) -> Result<Array<N>, ArrayError>;
}
