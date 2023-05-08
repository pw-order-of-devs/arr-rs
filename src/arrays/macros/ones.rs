/// Create an array of ones
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_ones!(8);
/// assert_eq!(array!([1, 1, 1, 1, 1, 1, 1, 1]).unwrap(), arr);
/// let arr: Array<f64> = array_ones!(2, 2, 2);
/// assert_eq!(array!([[[1, 1], [1, 1]], [[1, 1], [1, 1]]]).unwrap(), arr);
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
