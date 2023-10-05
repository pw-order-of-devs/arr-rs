/// Create an array
///
/// # Examples
///
/// ```
/// use arr_rs::prelude::*;
///
/// let arr = array!(i16, 1, 2, 3, 4, 5, 6, 7, 8).unwrap();
/// let arr = array!(i16, [1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
/// let arr = array!(i32, [[1, 2], [3, 4], [5, 6], [7, 8]]).unwrap();
/// let arr = array!(f64, [[1, 2, 3, 4], [5, 6, 7, 8]]).unwrap();
/// ```
#[macro_export]
macro_rules! array {
    (Tuple2<$t1:ty, $t2:ty>, $($x:expr),* $(,)*) => {{
        array_tuple!(Tuple2<$t1, $t2>, format!("{:?}", vec![$(vec![$x],)*]))
    }};
    (Tuple3<$t1:ty, $t2:ty, $t3:ty>, $($x:expr),* $(,)*) => {{
        array_tuple!(Tuple3<$t1, $t2, $t3>, format!("{:?}", vec![$($x,)*]))
    }};
    (List<$tt:ty>, $x:expr) => {{
        array_list!(List<$tt>, format!("{:?}", vec![$x]))
    }};
    (char, $($x:expr),* $(,)*) => {{
        array_char!(format!("{:?}", vec![$($x,)*]))
    }};
    (String, $($x:expr),* $(,)*) => {{
        array_string!(format!("{:?}", vec![$($x,)*]))
    }};
    ($tt:ty, $($x:expr),* $(,)*) => {{
        let string = format!("{:?}", vec![$($x,)*])
            .replace("\", \"", "\",\"")
            .replace("], [", "],[");
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 1;
        let ndim = if ndim == 0 { 1 } else { ndim };

        // get shape
        let shape = array_parse_shape!(ndim, string.clone());

        // get array elements
        let elems = string
            .replace("[", "")
            .replace("]", "")
            .replace(", ", ",")
            .replace("\"", "")
            .split_terminator(',')
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();

        // return array
        Array::<$tt>::new(elems, shape)
    }};
}
