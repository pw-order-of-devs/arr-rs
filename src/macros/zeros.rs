/// Create an array of zeros
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_zeros!(i32, 8);
/// assert_eq!(array!(i32, [0, 0, 0, 0, 0, 0, 0, 0]), arr);
/// let arr = array_zeros!(i32, 2, 2, 2);
/// assert_eq!(array!(i32, [[[0, 0], [0, 0]], [[0, 0], [0, 0]]]), arr);
/// ```
#[macro_export]
macro_rules! array_zeros {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_zeros! only accepts a flat list of elements");
    }};
    ($tt:ty, $([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_zeros! only accepts a flat list of elements");
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        Array::<$tt>::zeros(vec![$($x,)*])
    }};
}
