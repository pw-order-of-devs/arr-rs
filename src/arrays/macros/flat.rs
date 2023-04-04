/// Create a flat array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_flat!(1, 2, 3, 4, 5, 6, 7, 8);
/// assert_eq!(vec![8], arr.get_shape());
/// let arr: Array<f64> = array_flat!(1., 2., 3., 4., 5., 6., 7., 8.);
/// assert_eq!(vec![8], arr.get_shape());
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
