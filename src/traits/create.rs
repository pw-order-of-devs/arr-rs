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
    /// assert_eq!("[]", format!("{arr}"));
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
    /// let arr: Array<f64> = Array::zeros(vec![4]);
    /// assert_eq!("[0, 0, 0, 0]", format!("{arr}"));
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
    /// assert_eq!("[1, 1, 1, 1]", format!("{arr}"));
    fn ones(shape: Vec<usize>) -> Self;
}
