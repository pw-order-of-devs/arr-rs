/// Create a rand array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_rand!(8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr: Result<Array<f64>, ArrayError> = array_rand!(2, 2, 2);
/// assert_eq!(vec![2, 2, 2], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_rand {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($($x:expr),* $(,)*) => {{
        Array::rand(vec![$($x,)*])
    }};
}
