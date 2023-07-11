/// Create an array of ones
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_ones!(8);
/// assert_eq!(array!([1, 1, 1, 1, 1, 1, 1, 1]), arr);
/// let arr: Result<Array<f64>, ArrayError> = array_ones!(2, 2, 2);
/// assert_eq!(array!([[[1, 1], [1, 1]], [[1, 1], [1, 1]]]), arr);
/// ```
#[macro_export]
macro_rules! array_ones {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_ones! only accepts a flat list of elements");
    }};
    ($($x:expr),* $(,)*) => {{
        Array::ones(vec![$($x,)*])
    }};
}
