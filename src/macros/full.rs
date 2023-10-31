/// Create an array of `fill_value`
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
    ($tt:ty, $shape:expr, $fill:expr) => {{
        Array::<$tt>::full($shape.clone(), $fill)
    }};
}
