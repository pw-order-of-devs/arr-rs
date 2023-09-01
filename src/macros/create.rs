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
    ($tt:ty, $($x:expr),* $(,)*) => {{
        let string = format!("{:?}", vec![$($x,)*]).replace(" ", "");
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 1;
        let ndim = if ndim == 0 { 1 } else { ndim };

        // get shape
        let mut _string = string.clone();
        let mut shape = Vec::new();
        for i in (0..ndim).rev() {
            let tmp_str = _string.replace(&format!("{},{}", "]".repeat(i), "[".repeat(i)), "]#[");
            _string = _string[0 .. _string.find(&"]".repeat(i)).unwrap() + i].to_string();
            shape.push(tmp_str.split("#").count());
        };

        // get array elements
        let elems = string
            .replace("[", "").replace("]", "").replace(" ", "")
            .split_terminator(',')
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();

        // return array
        Array::<$tt>::new(elems, shape)
    }};
}
