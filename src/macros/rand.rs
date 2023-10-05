/// Create a rand array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_rand!(i32, 8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr = array_rand!(i32, 2, 2, 2);
/// assert_eq!(vec![2, 2, 2], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_rand {
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
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        Array::<$tt>::rand(vec![$($x,)*])
    }};
}
