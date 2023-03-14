use crate::base::base_type::Numeric;

/// Base Array structure
pub trait ArrayBase<N: Numeric> where Self: Sized + Clone +
std::fmt::Display + FromIterator<N> + IntoIterator<Item=N> {

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
    /// assert_eq!("[1, 2, 3, 4]", format!("{arr}"));
    /// let arr = arr.reshape(vec![2, 2]);
    /// assert_eq!("[[1, 2], [3, 4]]", format!("{arr}"));
    fn reshape(&self, shape: Vec<usize>) -> Self;

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
    fn product(&self) -> N;

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
    fn sum(&self) -> N;

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
    /// let arr_2: Array<f64> = Array::empty();
    /// let empty_2 = arr_2.is_empty();
    /// assert_eq!(true, empty_2);
    /// ```
    fn is_empty(&self) -> bool;

    /// Obtain the vector containing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]);
    /// assert_eq!(vec![1, 2, 3, 4], arr.get_elements());
    /// ```
    fn get_elements(&self) -> Vec<N>;

    /// Obtain the vector containing array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4], vec![4]);
    /// assert_eq!(vec![4], arr.get_shape());
    /// ```
    fn get_shape(&self) -> Vec<usize>;

    /// Return an index of element at the given coordinates
    ///
    /// # Arguments
    ///
    /// * `coords` - vector representing the coordinates of the element in array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]);
    ///
    /// let idx_1 = arr.index(&[0, 0, 0]);
    /// assert_eq!(0, idx_1);
    ///
    /// let idx_2 = arr.index(&[1, 0, 1]);
    /// assert_eq!(5, idx_2);
    ///
    /// let idx_3 = arr.index(&[1, 1, 1]);
    /// assert_eq!(7, idx_3);
    /// ```
    fn index(&self, coords: &[usize]) -> usize;

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
}
