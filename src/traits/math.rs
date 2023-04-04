use crate::traits::types::NumericOps;

/// ArrayTrait - Array Math functions
pub trait ArrayMath<N: NumericOps> where Self: Sized + Clone {

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
}
