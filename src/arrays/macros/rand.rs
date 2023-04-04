/// Create a rand array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_rand!(8);
/// assert_eq!(vec![8], arr.get_shape());
/// let arr: Array<f64> = array_rand!(2, 2, 2);
/// assert_eq!(vec![2, 2, 2], arr.get_shape());
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
