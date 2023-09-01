/// Create a rand array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_rand!(i32, 8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr = array_rand!(i32, 2, 2, 2);
/// assert_eq!(vec![2, 2, 2], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_rand {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        Array::<$tt>::rand(vec![$($x,)*])
    }};
}
