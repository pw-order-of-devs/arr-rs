use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::ArrayElement,
};

/// ArrayTrait - Array Iterable functions
pub trait ArrayIter<T: ArrayElement> where Self: Sized + Clone {

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
    fn for_each<F: FnMut(&T)>(&self, f: F) -> Result<(), ArrayError>;

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
    fn for_each_e<F: FnMut(usize, &T)>(&self, f: F) -> Result<(), ArrayError>;

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
    fn map<F: FnMut(&T) -> T>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn map_e<F: FnMut(usize, &T) -> T>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn filter<F: FnMut(&T) -> bool>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn filter_e<F: FnMut(usize, &T) -> bool>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn filter_map<F: FnMut(&T) -> Option<T>>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn filter_map_e<F: FnMut(usize, &T) -> Option<T>>(&self, f: F)-> Result<Array<T>, ArrayError>;

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
    fn fold<F: FnMut(&T, &T) -> T>(&self, init: T, f: F) -> Result<T, ArrayError>;
}
