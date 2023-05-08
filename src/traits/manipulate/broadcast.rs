use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::{
        numeric::Numeric,
        tuple_numeric::Tuple2,
    }
};

/// ArrayTrait - Array Broadcast functions
pub trait ArrayBroadcast<N: Numeric> where Self: Sized + Clone {

    /// Broadcast an array to a new shape
    ///
    /// # Arguments
    ///
    /// * `other` - other array for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<Tuple2<i32>> = Array::new(vec![
    ///     (1, 4), (1, 5), (1, 6),
    ///     (2, 4), (2, 5), (2, 6),
    ///     (3, 4), (3, 5), (3, 6)
    /// ].into_iter().map(Tuple2::from_tuple).collect(), vec![3, 3]).unwrap();
    ///
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let arr_2: Array<i32> = array!([[4, 5, 6]]).unwrap();
    ///
    /// let broadcast: Array<Tuple2<i32>> = arr_1.broadcast(&arr_2).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast(&self, other: &Self) -> Result<Array<Tuple2<N>>, ArrayError>;

    /// Broadcast an array to a new shape
    ///
    /// # Arguments
    ///
    /// * `other` - other array for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = Array::new(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], vec![3, 3]).unwrap();
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    ///
    /// let broadcast: Array<i32> = arr_1.broadcast_to(vec![3, 3]).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast_to(&self, shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Broadcast a list of arrays to a common shape
    ///
    /// # Arguments
    ///
    /// * `arrays` - list of arrays for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Vec<Array<i32>> = vec![
    ///     Array::new(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], vec![3, 3]).unwrap(),
    ///     Array::new(vec![4, 5, 6, 4, 5, 6, 4, 5, 6], vec![3, 3]).unwrap(),
    /// ];
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let arr_2: Array<i32> = array!([4, 5, 6]).unwrap();
    ///
    /// let broadcast: Vec<Array<i32>> = Array::broadcast_arrays(vec![arr_1 ,arr_2]).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast_arrays(arrays: Vec<Self>) -> Result<Vec<Self>, ArrayError>;
}

