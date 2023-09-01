/// Create an array of fill_value
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_full!(i32, vec![2, 2], 2);
/// assert_eq!(array!(i32, [[2, 2], [2, 2]]), arr);
/// let arr = array_full!(i32, vec![2, 2, 2], 2);
/// assert_eq!(array!(i32, [[[2, 2], [2, 2]], [[2, 2], [2, 2]]]), arr);
/// ```
#[macro_export]
macro_rules! array_full {
    ($tt:ty, $shape:expr, $fill:expr) => {{
        Array::<$tt>::full($shape.clone(), $fill)
    }};
}
