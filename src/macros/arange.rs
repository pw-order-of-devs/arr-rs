/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_arange!(i32, 0, 4).unwrap();
/// assert_eq!(array!(i32, [0, 1, 2, 3, 4]).unwrap(), arr);
/// let arr = array_arange!(i32, 0, 7, 2).unwrap();
/// assert_eq!(array!(i32, [0, 2, 4, 6]).unwrap(), arr);
/// ```
#[macro_export]
macro_rules! array_arange {
    ($tt:ty, $n:expr, $m:expr) => {
        Array::<$tt>::arange($n, $m, None)
    };
    ($tt:ty, $n:expr, $m:expr, $k:expr) => {{
        Array::<$tt>::arange($n, $m, Some($k))
    }};
}
