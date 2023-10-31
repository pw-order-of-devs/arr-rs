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
    (Tuple2<$t1:ty, $t2:ty>, ($val1:expr, $val2:expr)) => {{
        let val1 = $val1.to_string().parse().unwrap();
        let val2 = $val2.to_string().parse().unwrap();
        Array::<Tuple2<$t1, $t2>>::single(Tuple2(val1, val2))
    }};
    (Tuple3<$t1:ty, $t2:ty, $t3:ty>, ($val1:expr, $val2:expr, $val3:expr)) => {{
        let val1 = $val1.to_string().parse().unwrap();
        let val2 = $val2.to_string().parse().unwrap();
        let val3 = $val3.to_string().parse().unwrap();
        Array::<Tuple3<$t1, $t2, $t3>>::single(Tuple3(val1, val2, val3))
    }};
    (List<$tt:ty>, vec![$($x:expr),*]) => {{
        let values = vec![$($x),*]
            .into_iter()
            .map(|i| i.to_string().parse().unwrap())
            .collect::<Vec<_>>();
        Array::<List<$tt>>::single(List(values))
    }};
    ($tt:ty, $x:expr) => {{
        let value = $x.to_string().parse().unwrap();
        Array::<$tt>::single(value)
    }};
}
