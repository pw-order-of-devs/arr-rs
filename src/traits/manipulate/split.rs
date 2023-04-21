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
    /// let split = arr.split(3, None);
    /// assert_eq!(vec![array!(0, 1, 2), array!(3, 4, 5), array!(6, 7, 8)], split);
    ///
    /// let arr = array_arange!(0, 7);
    /// let split = arr.split(4, None);
    /// assert_eq!(vec![array!(0, 1), array!(2, 3), array!(4, 5), array!(6, 7)], split);
    /// ```
    fn split(&self, parts: usize, axis: Option<usize>) -> Vec<Self>;

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
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// let split = arr.hsplit(2);
    /// assert_eq!(vec![array!([[[0, 1]], [[4, 5]]]), array!([[[2, 3]], [[6, 7]]])], split);
    /// ```
    fn hsplit(&self, parts: usize) -> Vec<Self>;

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
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// let split = arr.vsplit(2);
    /// assert_eq!(vec![array!([[[0, 1], [2, 3]]]), array!([[[4, 5], [6, 7]]])], split);
    /// ```
    fn vsplit(&self, parts: usize) -> Vec<Self>;

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
    /// let arr = array_arange!(0, 7).reshape(vec![2, 2, 2]);
    /// let split = arr.dsplit(2);
    /// assert_eq!(vec![array!([[[0], [2]], [[4], [6]]]), array!([[[1], [3]], [[5], [7]]])], split);
    /// ```
    fn dsplit(&self, parts: usize) -> Vec<Self>;
}
