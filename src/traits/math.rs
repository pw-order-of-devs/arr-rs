use crate::traits::types::numeric::Numeric;

/// ArrayTrait - Array Math functions
pub trait ArrayMath<N: Numeric> where Self: Sized + Clone {

    /// Computes power of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// let multiplied = arr.power(2);
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]), multiplied);
    /// ```
    fn power(&self, value: N) -> Self;

    /// Multiplication of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
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
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// let sum = arr.sum();
    /// assert_eq!(10, sum);
    /// ```
    fn sum(&self) -> N;

    /// Cumulative sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// let sum = arr.cumsum();
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]), sum);
    /// ```
    fn cumsum(&self) -> Self;
}
