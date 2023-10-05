/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_identity!(i32, 2);
/// assert_eq!(vec![2, 2], arr.get_shape().unwrap());
/// let arr = array_identity!(i32, 4);
/// assert_eq!(vec![4, 4], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_identity {
    (String, $n:expr) => {{
        compile_error!("`String` macros are not supported")
    }};
    ($tt:ty, $n:expr) => {
        Array::<$tt>::identity($n)
    };
}
