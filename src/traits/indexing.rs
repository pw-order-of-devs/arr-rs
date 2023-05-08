use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Indexing functions
pub trait ArrayIndexing<N: Numeric> where Self: Sized + Clone {

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
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let idx_1 = arr.index_at(&[0, 0, 0]).unwrap();
    /// assert_eq!(0, idx_1);
    ///
    /// let idx_2 = arr.index_at(&[1, 0, 1]).unwrap();
    /// assert_eq!(5, idx_2);
    ///
    /// let idx_3 = arr.index_at(&[1, 1, 1]).unwrap();
    /// assert_eq!(7, idx_3);
    /// ```
    fn index_at(&self, coords: &[usize]) -> Result<usize, ArrayError>;

    /// Return coordinates at the given index of element
    ///
    /// # Arguments
    ///
    /// * `index` - index of element in flattened array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let coord_1 = arr.index_to_coord(0).unwrap();
    /// assert_eq!(vec![0, 0, 0], coord_1);
    ///
    /// let coord_2 = arr.index_to_coord(5).unwrap();
    /// assert_eq!(vec![1, 0, 1], coord_2);
    ///
    /// let coord_3 = arr.index_to_coord(7).unwrap();
    /// assert_eq!(vec![1, 1, 1], coord_3);
    /// ```
    fn index_to_coord(&self, idx: usize) -> Result<Vec<usize>, ArrayError>;

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
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let at_1 = arr.at(&[0, 0, 0]).unwrap();
    /// assert_eq!(1, at_1);
    ///
    /// let at_2 = arr.at(&[1, 0, 1]).unwrap();
    /// assert_eq!(6, at_2);
    ///
    /// let at_3 = arr.at(&[1, 1, 1]).unwrap();
    /// assert_eq!(8, at_3);
    /// ```
    fn at(&self, coords: &[usize]) -> Result<N, ArrayError>;

    /// Return a subarray of provided range
    ///
    /// # Arguments
    ///
    /// * `range` - starting and ending indices of elements to include in the subarray
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::new(vec![1,2,3,4,5,6,7,8], vec![8]).unwrap();
    /// let expected = Array::<i32>::new(vec![1,2,3,4], vec![4]).unwrap();
    /// let slice_1 = arr.slice(0..4).unwrap();
    /// assert_eq!(format!("{expected}"), format!("{slice_1}"));
    ///
    /// let arr = Array::<i32>::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// let expected = Array::<i32>::new(vec![1,2,3,4], vec![4]).unwrap();
    /// let slice_1 = arr.slice(0..1).unwrap();
    /// assert_eq!(format!("{expected}"), format!("{slice_1}"));
    /// ```
    fn slice(&self, range: std::ops::Range<usize>) -> Result<Self, ArrayError>;
}
