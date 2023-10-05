/// Create a flat array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_flat!(i32, 1, 2, 3, 4, 5, 6, 7, 8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr = array_flat!(f64, 1., 2., 3., 4., 5., 6., 7., 8.);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_flat {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        Array::<$tt>::flat(vec![$($x,)*])
    }};
}
