use num::NumCast;

/// Base Array structure
pub trait ArrayBase where Self: Sized + Clone {

    /// Creates new array
    ///
    /// # Arguments
    ///
    /// * `elements` - vector representing array elements
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![4]);
    /// // [1, 2, 3, 4]
    ///
    /// let arr_2 = Array::new(vec![1, 2, 3, 4], vec![2, 2]);
    /// // [[1, 2], [3, 4]]
    fn new<I, A>(elements: I, shape: Vec<usize>) -> Self
        where A: NumCast, I: IntoIterator<Item=A> + Clone;

    /// Creates new empty array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::empty();
    /// // []
    fn empty() -> Self;

    /// Creates new array of zeros
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::zeros(vec![4]);
    /// // [0, 0, 0, 0]
    fn zeros(shape: Vec<usize>) -> Self;

    /// Creates new array of ones
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::ones(vec![4]);
    /// // [1, 1, 1, 1]
    fn ones(shape: Vec<usize>) -> Self;

    /// Multiplication of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]);
    /// let multiplied = arr.product();
    /// assert_eq!(24, multiplied);
    /// ```
    fn product<F: NumCast>(&self) -> F;

    /// Sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]);
    /// let sum = arr.sum();
    /// assert_eq!(10, sum);
    /// ```
    fn sum<F: NumCast>(&self) -> F;

    /// Count of array dimensions
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4], vec![4]);
    /// let ndim_1 = arr_1.ndim();
    /// assert_eq!(1, ndim_1);
    ///
    /// let arr_2 = Array::new(vec![1,2,3,4], vec![2, 2]);
    /// let ndim_2 = arr_2.ndim();
    /// assert_eq!(2, ndim_2);
    /// ```
    fn ndim(&self) -> usize;

    /// Count of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]);
    /// let len = arr.len();
    /// assert_eq!(4, len);
    /// ```
    fn len(&self) -> usize;

    /// Check if array element count equals zero
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr_1 = Array::new(vec![1,2,3,4], vec![4]);
    /// let empty_1 = arr_1.is_empty();
    /// assert_eq!(false, empty_1);
    ///
    /// let arr_2 = Array::empty();
    /// let empty_2 = arr_2.is_empty();
    /// assert_eq!(true, empty_2);
    /// ```
    fn is_empty(&self) -> bool;
}
