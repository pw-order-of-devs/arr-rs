use crate::traits::types::numeric::Numeric;

/// ArrayTrait - Array Metadata functions
pub trait ArrayMeta<N: Numeric> where Self: Sized + Clone {

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
}
