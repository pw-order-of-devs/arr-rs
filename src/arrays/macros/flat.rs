/// Create a flat array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_flat!(1, 2, 3, 4, 5, 6, 7, 8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr: Result<Array<f64>, ArrayError> = array_flat!(1., 2., 3., 4., 5., 6., 7., 8.);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_flat {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($($x:expr),* $(,)*) => {{
        Array::flat(vec![$($x,)*])
    }};
}
