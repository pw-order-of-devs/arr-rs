/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_eye!(i32, 2, 3, 3);
/// assert_eq!(vec![2, 3], arr.get_shape().unwrap());
/// let arr = array_eye!(i32, 4, 5, 3);
/// assert_eq!(vec![4, 5], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_eye {
    ($tt:ty, $n:expr) => {
        array_eye!($tt, $n, $n, 0)
    };
    ($tt:ty, $n:expr, $m:expr) => {
        array_eye!($tt, $n, $m, 0)
    };
    ($tt:ty, $n:expr, $m:expr, $k:expr) => {{
        Array::<$tt>::eye($n, Some($m), Some($k))
    }};
}
