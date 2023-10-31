/// Create a flat array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array_flat!(i32, 1, 2, 3, 4, 5, 6, 7, 8);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// let arr = array_flat!(f64, 1., 2., 3., 4., 5., 6., 7., 8.);
/// assert_eq!(vec![8], arr.get_shape().unwrap());
/// ```
#[macro_export]
macro_rules! array_flat {
    ($([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    ($tt:ty, $([$($nested:expr),*]),* $(,)*) => {{
        compile_error!("array_flat! only accepts a flat list of elements");
    }};
    (Tuple2<$t1:ty, $t2:ty>, $($x:expr),* $(,)*) => {{
        array_tuple!(Tuple2<$t1, $t2>, format!("{:?}", vec![$(vec![$x],)*]))
    }};
    (Tuple3<$t1:ty, $t2:ty, $t3:ty>, $($x:expr),* $(,)*) => {{
        array_tuple!(Tuple3<$t1, $t2, $t3>, format!("{:?}", vec![$(vec![$x],)*]))
    }};
    (List<$tt:ty>, $($x:expr),* $(,)*) => {{
        array_list!(List<$tt>, format!("{:?}", vec![$(vec![$x],)*]))
    }};
    (char, $($x:expr),* $(,)*) => {{
        array_char!(format!("{:?}", vec![$(vec![$x],)*]))
    }};
    (String, $($x:expr),* $(,)*) => {{
        array_string!(format!("{:?}", vec![$(vec![$x],)*]))
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        array!($tt, vec![$($x,)*])
    }};
}
