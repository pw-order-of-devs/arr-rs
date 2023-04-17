/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_arange!(0, 5);
/// assert_eq!(array!([0, 1, 2, 3, 4]), arr);
/// let arr: Array<i32> = array_arange!(0, 8, 2);
/// assert_eq!(array!([0, 2, 4, 6]), arr);
/// ```
#[macro_export]
macro_rules! array_arange {
    ($n:expr, $m:expr) => {
        array_arange!($n, $m, 1)
    };
    ($n:expr, $m:expr, $k:expr) => {{
        Array::arange($n, $m, Some($k))
    }};
}