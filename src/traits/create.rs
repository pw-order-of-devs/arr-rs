use crate::arrays::Array;
use crate::traits::types::Numeric;

/// ArrayTrait - Array Create functions
pub trait ArrayCreate<N: Numeric> where Self: Sized + Clone {

    // ==== from data

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
    /// ```
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
    /// assert_eq!(array!([1, 2, 3, 4]), arr);
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
    fn zeros(shape: Vec<usize>) -> Self;

    /// Creates new array of zeros like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::zeros_like(&array![1, 2, 3, 4]);
    /// assert_eq!(array!([0, 0, 0, 0]), arr);
    /// ```
    fn zeros_like(other: &Self) -> Self;

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
    /// ```
    fn ones(shape: Vec<usize>) -> Self;

    /// Creates new array of ones like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::ones_like(&array![1, 2, 3, 4]);
    /// assert_eq!(array!([1, 1, 1, 1]), arr);
    /// ```
    fn ones_like(other: &Self) -> Self;

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
    /// ```
    fn full(shape: Vec<usize>, fill_value: N) -> Self;

    /// Creates new array of fill_value like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    /// * `fill_value` - value to fill the array with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::full_like(&array![1, 2, 3, 4], 2.);
    /// assert_eq!(array!([2, 2, 2, 2]), arr);
    /// ```
    fn full_like(other: &Self, fill_value: N) -> Self;

    // ==== from range

    /// Creates new array from range
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `step` - spacing between values. optional, defaults to 1
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::arange(0., 5., None);
    /// assert_eq!(array!([0., 1., 2., 3., 4.]), arr);
    /// ```
    fn arange(start: N, stop: N, step: Option<N>) -> Self;

    /// Creates new array of numbers evenly spaced over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::linspace(0., 5., Some(3), None);
    /// assert_eq!(array!([0., 2.5, 5.]), arr);
    /// ```
    fn linspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Self;

    /// Creates new array of numbers evenly spaced over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::linspace_a(&array!([0., 2.]), &array!([4., 6.]), Some(3), None);
    /// assert_eq!(array!([[0., 2.], [2., 4.], [4., 6.]]), arr);
    /// let arr: Array<f64> = Array::linspace_a(&array!([[0., 2.], [2., 4.]]), &array!([[4., 6.], [6., 8.]]), Some(3), None);
    /// assert_eq!(array!([[[0., 2.], [2., 4.]], [[2., 4.], [4., 6.]], [[4., 6.], [6., 8.]]]), arr);
    /// ```
    fn linspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Self;

    /// Creates new array of numbers evenly spaced on a log scale over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    /// * `base` - the base of the log space. optional, defaults to 10
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::logspace(0., 5., Some(3), None, None);
    /// assert_eq!(array!([1., 316.22776601683796, 100000.]), arr);
    /// ```
    fn logspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>, base: Option<usize>) -> Self;

    /// Creates new array of numbers evenly spaced on a log scale over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    /// * `base` - the base of the log space. optional, defaults to 10
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::logspace_a(&array!([0., 2.]), &array!([4., 6.]), Some(3), None, None);
    /// assert_eq!(array!([[1., 100.], [100., 10000.], [10000., 1000000.]]), arr);
    /// let arr: Array<f64> = Array::logspace_a(&array!([[0., 2.], [2., 4.]]), &array!([[4., 6.], [6., 8.]]), Some(3), None, None);
    /// assert_eq!(array!([[[1., 100.], [100., 10000.]], [[100., 10000.], [10000., 1000000.]], [[10000., 1000000.], [1000000., 100000000.]]]), arr);
    /// ```
    fn logspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>, base: Option<&Array<usize>>) -> Self;

    /// Creates new array of numbers evenly spaced on a log scale (a geometric progression) over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<f64> = array!([1., 10., 100., 1000.]);
    /// let arr: Array<f64> = Array::geomspace(1., 1000., Some(4), None);
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Self;

    /// Creates new array of numbers evenly spaced on a log scale (a geometric progression) over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<f64> = array!([[1., 10.], [10., 100.], [100., 1000.], [1000., 10000.]]);
    /// let arr: Array<f64> = Array::geomspace_a(&array!([1., 10.]), &array!([1000., 10000.]), Some(4), None);
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// let expected: Array<f64> = array!([[[1., 10.], [10., 100.]], [[10., 100.], [100., 1000.]], [[100., 1000.], [1000., 10000.]]]);
    /// let arr: Array<f64> = Array::geomspace_a(&array!([[1., 10.], [10., 100.]]), &array!([[100., 1000.], [1000., 10000.]]), Some(3), None);
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Self;

    // ==== matrices

    /// Extract a diagonal or construct a diagonal array
    ///
    /// # Arguments
    ///
    /// * `data` - input array
    /// * `k` - chosen diagonal
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]);
    /// let arr: Array<i32> = Array::diag(&array![1, 2, 3, 4], None);
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([1, 2, 3, 4]);
    /// let arr: Array<i32> = Array::diag(&array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]), None);
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([0, 0, 0]);
    /// let arr: Array<i32> = Array::diag(&array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]), Some(1));
    /// assert_eq!(expected, arr);
    /// ```
    fn diag(data: &Self, k: Option<isize>) -> Self;

    /// Construct a diagonal array for flattened input
    ///
    /// # Arguments
    ///
    /// * `data` - input array
    /// * `k` - chosen diagonal
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]);
    /// let arr: Array<i32> = Array::diagflat(&array![1, 2, 3, 4], None);
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]);
    /// let arr: Array<i32> = Array::diagflat(&array![[1, 2], [3, 4]], None);
    /// assert_eq!(expected, arr);
    /// ```
    fn diagflat(data: &Self, k: Option<isize>) -> Self;
}
