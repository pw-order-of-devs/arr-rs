/// Create an array of zeros
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_zeros!(8);
/// assert_eq!(array!([0, 0, 0, 0, 0, 0, 0, 0]).unwrap(), arr);
/// let arr: Array<f64> = array_zeros!(2, 2, 2);
/// assert_eq!(array!([[[0, 0], [0, 0]], [[0, 0], [0, 0]]]).unwrap(), arr);
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
