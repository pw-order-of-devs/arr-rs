/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_identity!(2);
/// assert_eq!(vec![2, 2], arr.get_shape().unwrap());
/// let arr: Result<Array<f64>, ArrayError> = array_identity!(4);
/// assert_eq!(vec![4, 4], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_identity {
    ($n:expr) => {
        Array::identity($n)
    };
}
