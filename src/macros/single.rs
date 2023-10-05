/// Create a single array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_single!(i32, 8);
/// assert_eq!(vec![1], arr.get_shape().unwrap());
/// let arr = array_single!(f64, 8.);
/// assert_eq!(vec![1], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_single {
    ($tt:ty, $x:expr) => {{
        Array::<$tt>::single($x)
    }};
}
