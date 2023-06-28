use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::ArrayElement,
};

/// ArrayTrait - Array Split functions
pub trait ArraySplit<T: ArrayElement> where Self: Sized + Clone {

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
    /// let split = arr.array_split(3, None).unwrap();
    /// assert_eq!(vec![array_flat!(0, 1, 2).unwrap(), array_flat!(3, 4, 5).unwrap(), array_flat!(6, 7).unwrap()], split);
    ///
    /// let arr = array_arange!(0, 8);
    /// let split = arr.array_split(4, None).unwrap();
    /// assert_eq!(vec![array_flat!(0, 1, 2).unwrap(), array_flat!(3, 4).unwrap(), array_flat!(5, 6).unwrap(), array_flat!(7, 8).unwrap()], split);
    /// ```
    fn array_split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays of equal size
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
    /// let arr = array_arange!(0, 8);
    /// let split = arr.split(3, None).unwrap();
    /// assert_eq!(vec![array_flat!(0, 1, 2).unwrap(), array_flat!(3, 4, 5).unwrap(), array_flat!(6, 7, 8).unwrap()], split);
    ///
    /// let arr = array_arange!(0, 7);
    /// let split = arr.split(4, None).unwrap();
    /// assert_eq!(vec![array_flat!(0, 1).unwrap(), array_flat!(2, 3).unwrap(), array_flat!(4, 5).unwrap(), array_flat!(6, 7).unwrap()], split);
    /// ```
    fn split(&self, parts: usize, axis: Option<usize>) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays horizontally (column-wise)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]).unwrap();
    /// let split = arr.hsplit(2).unwrap();
    /// assert_eq!(vec![array!([[[0, 1]], [[4, 5]]]).unwrap(), array!([[[2, 3]], [[6, 7]]]).unwrap()], split);
    /// ```
    fn hsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays vertically (row-wise)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]).unwrap();
    /// let split = arr.vsplit(2).unwrap();
    /// assert_eq!(vec![array!([[[0, 1], [2, 3]]]).unwrap(), array!([[[4, 5], [6, 7]]]).unwrap()], split);
    /// ```
    fn vsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;

    /// Split an array into multiple sub-arrays along the 3rd axis (depth)
    ///
    /// # Arguments
    ///
    /// * `parts` - indices defining how to split the array
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]).unwrap();
    /// let split = arr.dsplit(2).unwrap();
    /// assert_eq!(vec![array!([[[0], [2]], [[4], [6]]]).unwrap(), array!([[[1], [3]], [[5], [7]]]).unwrap()], split);
    /// ```
    fn dsplit(&self, parts: usize) -> Result<Vec<Array<T>>, ArrayError>;
}
