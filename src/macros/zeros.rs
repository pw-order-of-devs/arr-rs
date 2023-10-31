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
    (Tuple2<$t1:ty, $t2:ty>, $($tt:tt)*) => {{
        compile_error!("only `Numeric` types are supported")
    }};
    (Tuple3<$t1:ty, $t2:ty, $t3:ty>, $($tt:tt)*) => {{
        compile_error!("only `Numeric` types are supported")
    }};
    (List<$t1:ty>, $($tt:tt)*) => {{
        compile_error!("only `Numeric` types are supported")
    }};
    (char, $($tt:tt)*) => {{
        compile_error!("only `Numeric` types are supported")
    }};
    (String, $($tt:tt)*) => {{
        compile_error!("only `Numeric` types are supported")
    }};
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
