/// Create new 2d array with ones on the diagonal and zeros elsewhere
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Array<i32> = array_eye!(2, 3, 3);
/// assert_eq!(vec![2, 3], arr.get_shape());
/// let arr: Array<f64> = array_eye!(4, 5, 3);
/// assert_eq!(vec![4, 5], arr.get_shape());
/// ```
#[macro_export]
macro_rules! array_eye {
    ($n:expr) => {
        array_eye!($n, $n, 0)
    };
    ($n:expr, $m:expr) => {
        array_eye!($n, $m, 0)
    };
    ($n:expr, $m:expr, $k:expr) => {{
        Array::eye($n, Some($m), Some($k))
    }};
}
