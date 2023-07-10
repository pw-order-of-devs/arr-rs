/// Create an array of fill_value
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr: Result<Array<i32>, ArrayError> = array_full!(vec![2, 2], 2);
/// assert_eq!(array!([[2, 2], [2, 2]]), arr);
/// let arr: Result<Array<i32>, ArrayError> = array_full!(vec![2, 2, 2], 2);
/// assert_eq!(array!([[[2, 2], [2, 2]], [[2, 2], [2, 2]]]), arr);
/// ```
#[macro_export]
macro_rules! array_full {
    ($shape:expr, $fill:expr) => {{
        Array::full($shape.clone(), $fill)
    }};
}
