use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Math functions
pub trait ArrayMath<N: Numeric> where Self: Sized + Clone {

    /// Computes power of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]).unwrap();
    /// let multiplied = arr.power(2).unwrap();
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]).unwrap(), multiplied);
    /// ```
    fn power(&self, value: N) -> Result<Array<N>, ArrayError>;

    /// Multiplication of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// let multiplied = arr.product().unwrap();
    /// assert_eq!(24, multiplied);
    /// ```
    fn product(&self) -> Result<N, ArrayError>;

    /// Sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// let sum = arr.sum().unwrap();
    /// assert_eq!(10, sum);
    /// ```
    fn sum(&self) -> Result<N, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]).unwrap();
    /// let sum = arr.cumsum().unwrap();
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]).unwrap(), sum);
    /// ```
    fn cumsum(&self) -> Result<Array<N>, ArrayError>;
}
