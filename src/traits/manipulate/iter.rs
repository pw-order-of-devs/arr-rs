use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Array Iterable functions
pub trait ArrayIter<N: Numeric> where Self: Sized + Clone {

    /// Loop over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.for_each(|item| println!("{item}")).unwrap();
    /// ```
    fn for_each<F: FnMut(&N)>(&self, f: F) -> Result<(), ArrayError>;

    /// Loop over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.for_each_e(|idx, item| println!("{idx}:{item}")).unwrap();
    /// ```
    fn for_each_e<F: FnMut(usize, &N)>(&self, f: F) -> Result<(), ArrayError>;

    /// Map over array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.map(|item| item * 2).unwrap();
    /// ```
    fn map<F: FnMut(&N) -> N>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Map over enumerated array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.map_e(|idx, item| item * idx as i32).unwrap();
    /// ```
    fn map_e<F: FnMut(usize, &N) -> N>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Filter over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter(|item| item % 2 == 0).unwrap();
    /// ```
    fn filter<F: FnMut(&N) -> bool>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Filter over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_e(|idx, item| item % (idx + 1) as i32 == 0).unwrap();
    /// ```
    fn filter_e<F: FnMut(usize, &N) -> bool>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Filter and map over array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_map(|item| if item % 2 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map<F: FnMut(&N) -> Option<N>>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Filter and map over enumerated array elements
    /// Returns a flat filtered array
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.filter_map_e(|idx, item| if item % (idx + 1) as i32 == 0 { Some(*item) } else { None }).unwrap();
    /// ```
    fn filter_map_e<F: FnMut(usize, &N) -> Option<N>>(&self, f: F)-> Result<Array<N>, ArrayError>;

    /// Fold elements of array elements
    ///
    /// # Arguments
    ///
    /// * `f` - function to be called on each array element
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 4]).unwrap();
    /// arr.fold(0, |a, b| a + b).unwrap();
    /// arr.fold(1, |a, b| a * b).unwrap();
    /// ```
    fn fold<F: FnMut(&N, &N) -> N>(&self, init: N, f: F) -> Result<N, ArrayError>;
}
