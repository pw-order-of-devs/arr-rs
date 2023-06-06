use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    types::numeric::Numeric,
};

/// ArrayTrait - Binary Array operations
pub trait ArrayBinary<N: Numeric> where Self: Sized + Clone {

    /// Compute the bit-wise AND of two arrays element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::<i32>::flat(vec![1]), array!([13]).bitwise_and(&array!([17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![0, 1]), array!([11, 7]).bitwise_and(&array!([4, 25]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![2, 4, 16]), array!([2, 5, 255]).bitwise_and(&array!([3, 14, 16]).unwrap()));
    /// ```
    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute the bit-wise OR of two arrays element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::<i32>::flat(vec![29]), array!([13]).bitwise_or(&array!([16]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![33, 6]), array!([33, 4]).bitwise_or(&array!([1, 2]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![6, 5, 255]), array!([2, 5, 255]).bitwise_or(&array!([4, 4, 4]).unwrap()));
    /// ```
    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute the bit-wise XOR of two arrays element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::<i32>::flat(vec![28]), array!([13]).bitwise_xor(&array!([17]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26]), array!([31]).bitwise_xor(&array!([5]).unwrap()));
    /// assert_eq!(Array::<i32>::flat(vec![26, 5]), array!([31, 3]).bitwise_xor(&array!([5, 6]).unwrap()));
    /// ```
    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Compute bit-wise inversion, or bit-wise NOT, element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!([13]).bitwise_not());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!([13]).bitwise_not());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!([13]).bitwise_not());
    /// ```
    fn bitwise_not(&self) -> Result<Array<N>, ArrayError>;

    /// Compute bit-wise inversion, or bit-wise NOT, element-wise. Alias on bitwise_not
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::<u8>::flat(vec![242]), array!([13]).invert());
    /// assert_eq!(Array::<u16>::flat(vec![65522]), array!([13]).invert());
    /// assert_eq!(Array::<i32>::flat(vec![-14]), array!([13]).invert());
    /// ```
    fn invert(&self) -> Result<Array<N>, ArrayError>;
}
