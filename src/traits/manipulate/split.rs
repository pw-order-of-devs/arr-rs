use crate::traits::types::numeric::Numeric;

/// ArrayTrait - Array Split functions
pub trait ArraySplit<N: Numeric> where Self: Sized + Clone {

    /// Split an array into multiple sub-arrays
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    /// * `axis` - the axis along which to split. optional, defaults to 0
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7);
    /// let split = arr.array_split(3, None);
    /// assert_eq!(vec![array!(0, 1, 2), array!(3, 4, 5), array!(6, 7)], split);
    ///
    /// let arr = array_arange!(0, 8);
    /// let split = arr.array_split(4, None);
    /// assert_eq!(vec![array!(0, 1, 2), array!(3, 4), array!(5, 6), array!(7, 8)], split);
    /// ```
    fn array_split(&self, parts: usize, axis: Option<usize>) -> Vec<Self>;
}
