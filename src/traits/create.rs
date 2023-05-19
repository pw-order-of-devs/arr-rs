use crate::arrays::Array;
use crate::traits::errors::ArrayError;
use crate::traits::types::numeric::Numeric;

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
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![4]).unwrap();
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    ///
    /// let arr = Array::new(vec![1, 2, 3, 4], vec![2, 2]).unwrap();
    /// assert_eq!("[[1, 2], [3, 4]]", format!("{arr}"));
    /// assert_eq!("[[1, 2], \n [3, 4]]", format!("{arr:#}"));
    /// ```
    fn new(elements: Vec<N>, shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new array with single element
    ///
    /// # Arguments
    ///
    /// * `element` - array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::single(1);
    /// assert_eq!(vec![1], arr.get_elements().unwrap());
    /// assert_eq!(vec![1], arr.get_shape().unwrap());
    /// ```
    fn single(element: N) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([1, 2, 3, 4]), Array::<i32>::flat(vec![1, 2, 3, 4]));
    /// ```
    fn flat(elements: Vec<N>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(4, Array::<f64>::rand(vec![4]).len().unwrap());
    /// assert_eq!(64, Array::<f64>::rand(vec![4, 4, 4]).len().unwrap());
    /// ```
    fn rand(shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new empty array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert!(Array::<i32>::empty().is_empty().unwrap());
    /// ```
    fn empty() -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0]]), Array::<i32>::eye(2, Some(3), Some(0)));
    /// assert_eq!(array!([[0, 1, 0], [0, 0, 1]]), Array::<i32>::eye(2, Some(3), Some(1)));
    /// ```
    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([[1, 0], [0, 1]]), Array::<i32>::identity(2));
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), Array::<i32>::identity(3));
    /// ```
    fn identity(dim: usize) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([0, 0, 0, 0]), Array::<f64>::zeros(vec![4]));
    /// ```
    fn zeros(shape: Vec<usize>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([0, 0, 0, 0]), Array::<f64>::zeros_like(&array![1, 2, 3, 4].unwrap()));
    /// ```
    fn zeros_like(other: &Self) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([1, 1, 1, 1]), Array::<f64>::ones(vec![4]));
    /// ```
    fn ones(shape: Vec<usize>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([1, 1, 1, 1]), Array::<f64>::ones_like(&array![1, 2, 3, 4].unwrap()));
    /// ```
    fn ones_like(other: &Self) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([2, 2, 2, 2]), Array::<f64>::full(vec![4], 2.));
    /// ```
    fn full(shape: Vec<usize>, fill_value: N) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([2, 2, 2, 2]), Array::<f64>::full_like(&array![1, 2, 3, 4].unwrap(), 2.));
    /// ```
    fn full_like(other: &Self, fill_value: N) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([0., 1., 2., 3., 4.]), Array::arange(0., 4., None));
    /// ```
    fn arange(start: N, stop: N, step: Option<N>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([0., 2.5, 5.]), Array::<f64>::linspace(0., 5., Some(3), None));
    /// ```
    fn linspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

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
    /// let arr: Array<f64> = Array::linspace_a(&array!([0., 2.]).unwrap(), &array!([4., 6.]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(array!([[0., 2.], [2., 4.], [4., 6.]]).unwrap(), arr);
    /// let arr: Array<f64> = Array::linspace_a(&array!([[0., 2.], [2., 4.]]).unwrap(), &array!([[4., 6.], [6., 8.]]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(array!([[[0., 2.], [2., 4.]], [[2., 4.], [4., 6.]], [[4., 6.], [6., 8.]]]).unwrap(), arr);
    /// ```
    fn linspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

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
    /// assert_eq!(array!([1., 316.22776601683796, 100000.]), Array::<f64>::logspace(0., 5., Some(3), None, None));
    /// ```
    fn logspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>, base: Option<usize>) -> Result<Self, ArrayError>;

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
    /// let arr: Array<f64> = Array::logspace_a(&array!([0., 2.]).unwrap(), &array!([4., 6.]).unwrap(), Some(3), None, None).unwrap();
    /// assert_eq!(array!([[1., 100.], [100., 10000.], [10000., 1000000.]]).unwrap(), arr);
    /// let arr: Array<f64> = Array::logspace_a(&array!([[0., 2.], [2., 4.]]).unwrap(), &array!([[4., 6.], [6., 8.]]).unwrap(), Some(3), None, None).unwrap();
    /// assert_eq!(array!([[[1., 100.], [100., 10000.]], [[100., 10000.], [10000., 1000000.]], [[10000., 1000000.], [1000000., 100000000.]]]).unwrap(), arr);
    /// ```
    fn logspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>, base: Option<&Array<usize>>) -> Result<Self, ArrayError>;

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
    /// let expected: Array<f64> = array!([1., 10., 100., 1000.]).unwrap();
    /// let arr: Array<f64> = Array::geomspace(1., 1000., Some(4), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

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
    /// let expected: Array<f64> = array!([[1., 10.], [10., 100.], [100., 1000.], [1000., 10000.]]).unwrap();
    /// let arr: Array<f64> = Array::geomspace_a(&array!([1., 10.]).unwrap(), &array!([1000., 10000.]).unwrap(), Some(4), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// let expected: Array<f64> = array!([[[1., 10.], [10., 100.]], [[10., 100.], [100., 1000.]], [[100., 1000.], [1000., 10000.]]]).unwrap();
    /// let arr: Array<f64> = Array::geomspace_a(&array!([[1., 10.], [10., 100.]]).unwrap(), &array!([[100., 1000.], [1000., 10000.]]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

    // ==== matrices

    /// Construct an array with ones at and below the given diagonal and zeros elsewhere
    ///
    /// # Arguments
    ///
    /// * `n` - number of rows
    /// * `m` - number of columns. optional, defaults to `n`
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0], [1, 1]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::tri(2, Some(2), None).unwrap());
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0], [1, 1, 0], [1, 1, 1]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::tri(3, Some(3), None).unwrap());
    /// ```
    fn tri(n: usize, m: Option<usize>, k: Option<isize>) -> Result<Self, ArrayError>;
}

/// ArrayTrait - Array Create functions
pub trait ArrayCreateFrom<N: Numeric> where Array<N>: Sized + Clone {

    // matrices

    /// Extract a diagonal or construct a diagonal array
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr: Array<i32> = Array::diag(&array![1, 2, 3, 4].unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([1, 2, 3, 4]).unwrap();
    /// let arr: Array<i32> = Array::diag(&array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([0, 0, 0]).unwrap();
    /// let arr: Array<i32> = Array::diag(&array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap(), Some(1)).unwrap();
    /// assert_eq!(expected, arr);
    /// ```
    fn diag(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Construct a diagonal array for flattened input
    ///
    /// # Arguments
    ///
    /// * `data` - input array
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr: Array<i32> = Array::diagflat(&array![1, 2, 3, 4].unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [0, 2, 0, 0], [0, 0, 3, 0], [0, 0, 0, 4]]).unwrap();
    /// let arr: Array<i32> = Array::diagflat(&array!([[1, 2], [3, 4]]).unwrap(), None).unwrap();
    /// assert_eq!(expected, arr);
    /// ```
    fn diagflat(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return a copy of an array with elements above the k-th diagonal zeroed.
    /// For arrays with ndim exceeding 2, tril will apply to the final two axes.
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array_arange!(1, 8).reshape(vec![2, 4]).unwrap();
    /// let expected: Array<i32> = array!([[1, 0, 0, 0], [5, 6, 0, 0]]).unwrap();
    /// assert_eq!(expected, arr.tril(None).unwrap());
    ///
    /// let arr: Array<i32> = array_arange!(1, 8).reshape(vec![2, 2, 2]).unwrap();
    /// let expected: Array<i32> = array!([[[1, 0], [3, 4]], [[5, 0], [7, 8]]]).unwrap();
    /// assert_eq!(expected, arr.tril(None).unwrap());
    /// ```
    fn tril(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return a copy of an array with elements below the k-th diagonal zeroed.
    /// For arrays with ndim exceeding 2, triu will apply to the final two axes.
    ///
    /// # Arguments
    ///
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array_arange!(1, 8).reshape(vec![2, 4]).unwrap();
    /// let expected: Array<i32> = array!([[1, 2, 3, 4], [0, 6, 7, 8]]).unwrap();
    /// assert_eq!(expected, arr.triu(None).unwrap());
    ///
    /// let arr: Array<i32> = array_arange!(1, 8).reshape(vec![2, 2, 2]).unwrap();
    /// let expected: Array<i32> = array!([[[1, 2], [0, 4]], [[5, 6], [0, 8]]]).unwrap();
    /// assert_eq!(expected, arr.triu(None).unwrap());
    /// ```
    fn triu(&self, k: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Generate a Vandermonde matrix
    ///
    /// # Arguments
    ///
    /// * `n` - number of columns in the output. optional, by default square array is returned
    /// * `increasing` - order of the powers of the columns. optional, defaults to false
    /// if true, the powers increase from left to right, if false, they are reversed.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = array!([1, 2, 3, 4]).unwrap();
    /// let expected: Array<i32> = array!([[1, 1, 1, 1], [8, 4, 2, 1], [27, 9, 3, 1], [64, 16, 4, 1]]).unwrap();
    /// assert_eq!(expected, arr.vander(None, Some(false)).unwrap());
    ///
    /// let arr: Array<i32> = array!([1, 2, 3, 4]).unwrap();
    /// let expected: Array<i32> = array!([[1, 1, 1, 1], [1, 2, 4, 8], [1, 3, 9, 27], [1, 4, 16, 64]]).unwrap();
    /// assert_eq!(expected, arr.vander(None, Some(true)).unwrap());
    /// ```
    fn vander(&self, n: Option<usize>, increasing: Option<bool>) -> Result<Array<N>, ArrayError>;
}
