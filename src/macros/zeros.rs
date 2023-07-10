/// Create an array of zeros
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_zeros!(8);
/// assert_eq!(array!([0, 0, 0, 0, 0, 0, 0, 0]), arr);
/// let arr: Result<Array<f64>, ArrayError> = array_zeros!(2, 2, 2);
/// assert_eq!(array!([[[0, 0], [0, 0]], [[0, 0], [0, 0]]]), arr);
/// ```
#[macro_export]
macro_rules! array_zeros {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_zeros! only accepts a flat list of elements");
    }};
    ($($x:expr),* $(,)*) => {{
        Array::zeros(vec![$($x,)*])
    }};
}
