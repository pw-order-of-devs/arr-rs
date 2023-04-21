/// Array Axis functions
pub mod axis;
/// Array Broadcast functions
pub mod broadcast;
/// Array Split functions
pub mod split;

use crate::traits::types::numeric::Numeric;

/// ArrayTrait - Array Manipulate functions
pub trait ArrayManipulate<N: Numeric> where Self: Sized + Clone {

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
    /// let arr: Array<f64> = Array::new(vec![1., 2., 3., 4.], vec![4]);
    /// assert_eq!(array!([1, 2, 3, 4]), arr);
    /// let arr = arr.reshape(vec![2, 2]);
    /// assert_eq!(array!([[1, 2], [3, 4]]), arr);
    /// ```
    fn reshape(&self, shape: Vec<usize>) -> Self;

    /// Return a contiguous flattened array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = vec![8];
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// assert_eq!(expected, arr_1.ravel().get_shape());
    ///
    /// let arr_2 = Array::new(vec![1,2,3,4,5,6,7,8], vec![4, 2]);
    /// assert_eq!(expected, arr_2.ravel().get_shape());
    ///
    /// let arr_3 = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]);
    /// assert_eq!(expected, arr_3.ravel().get_shape());
    /// ```
    fn ravel(&self) -> Self;

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
    /// let arr = Array::new(vec![1], vec![1]);
    /// assert_eq!(array!([[1]]), arr.atleast(2));
    /// assert_eq!(array!([[[1]]]), arr.atleast(3));
    /// ```
    fn atleast(&self, n: usize) -> Self;

    /// Loop over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.for_each(|item| println!("{item}"));
    /// ```
    fn for_each<F: FnMut(&N)>(&self, f: F);

    /// Loop over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.for_each_e(|idx, item| println!("{idx}:{item}"));
    /// ```
    fn for_each_e<F: FnMut(usize, &N)>(&self, f: F);

    /// Map over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.map(|item| item * 2);
    /// ```
    fn map<F: FnMut(&N) -> N>(&self, f: F) -> Self;

    /// Map over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.map_e(|idx, item| item * idx as i32);
    /// ```
    fn map_e<F: FnMut(usize, &N) -> N>(&self, f: F) -> Self;

    /// Filter over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.filter(|item| item % 2 == 0);
    /// ```
    fn filter<F: FnMut(&N) -> bool>(&self, f: F) -> Self;

    /// Filter over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.filter_e(|idx, item| item % (idx + 1) as i32 == 0);
    /// ```
    fn filter_e<F: FnMut(usize, &N) -> bool>(&self, f: F) -> Self;

    /// Filter and map over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.filter_map(|item| if item % 2 == 0 { Some(*item) } else { None });
    /// ```
    fn filter_map<F: FnMut(&N) -> Option<N>>(&self, f: F) -> Self;

    /// Filter and map over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.filter_map_e(|idx, item| if item % (idx + 1) as i32 == 0 { Some(*item) } else { None });
    /// ```
    fn filter_map_e<F: FnMut(usize, &N) -> Option<N>>(&self, f: F) -> Self;

    /// Fold elements of array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]);
    /// arr.fold(0, |a, b| a + b);
    /// arr.fold(1, |a, b| a * b);
    /// ```
    fn fold<F: FnMut(&N, &N) -> N>(&self, init: N, f: F) -> N;
}
