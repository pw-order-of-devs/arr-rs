use crate::traits::types::Numeric;

/// ArrayTrait - Array Create functions
pub trait ArrayCreate<N: Numeric> where Self: Sized + Clone {

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
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    ///
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![2, 2]);
    /// assert_eq!("[[1, 2], [3, 4]]", format!("{arr}"));
    /// assert_eq!("[[1, 2], \n [3, 4]]", format!("{arr:#}"));
    fn new(elements: Vec<N>, shape: Vec<usize>) -> Self;

    /// Creates new flat array
    ///
    /// # Arguments
    ///
    /// * `elements` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    fn flat(elements: Vec<N>) -> Self;

    /// Creates new array with random elements from (0 ..= 1) range
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
    /// let arr = Array::<f64>::rand(vec![4]);
    /// assert_eq!(4, arr.len());
    ///
    /// let arr = Array::<f64>::rand(vec![4, 4, 4]);
    /// assert_eq!(64, arr.len());
    fn rand(shape: Vec<usize>) -> Self;

    /// Creates new empty array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::empty();
    /// assert!(arr.is_empty());
    fn empty() -> Self;

    /// Creates new 2d array with ones on the diagonal and zeros elsewhere
    ///
    /// # Arguments
    ///
    /// * `n` - number of rows
    /// * `m` - number of columns. optional, defaulted to n
    /// * `k` - index of diagonal. optional, defaulted to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::eye(2, Some(3), Some(0));
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0]]), arr);
    /// let arr: Array<i32> = Array::eye(2, Some(3), Some(1));
    /// assert_eq!(array!([[0, 1, 0], [0, 0, 1]]), arr);
    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Self;

    /// Creates new identity 2d array
    ///
    /// # Arguments
    ///
    /// * `n` - numbers of rows and columns of resulting array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::identity(2);
    /// assert_eq!(array!([[1, 0], [0, 1]]), arr);
    /// let arr: Array<i32> = Array::identity(3);
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), arr);
    fn identity(dim: usize) -> Self;

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
    /// let arr: Array<f64> = Array::zeros(vec![4]);
    /// assert_eq!(array!([0, 0, 0, 0]), arr);
    fn zeros(shape: Vec<usize>) -> Self;

    /// Creates new array of ones
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::ones(vec![4]);
    /// assert_eq!(array!([1, 1, 1, 1]), arr);
    fn ones(shape: Vec<usize>) -> Self;

    /// Creates new array of fill_value
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array shape
    /// * `fill_value` - value to fill the array with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::full(vec![4], 2.);
    /// assert_eq!(array!([2, 2, 2, 2]), arr);
    fn full(shape: Vec<usize>, fill_value: N) -> Self;
}
