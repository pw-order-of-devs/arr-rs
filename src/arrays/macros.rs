/// Create an array
///
/// # Examples
///
/// ```
/// use arr_rs::array;
/// use arr_rs::prelude::*;
///
/// array![[1,2],[3,4],[5,6],[7,8]];
/// array![[1,2,3,4],[5,6,7,8]];
/// array![[[1,2],[3,4]],[[5,6],[7,8]]];
/// ```
#[macro_export]
macro_rules! array {
    ($($x:expr),* $(,)*) => {{
        let string = format!("{:?}", vec![$($x,)*]).replace(" ", "");
        let ndim = string.find(|p| p != '[').unwrap_or(0);

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
            .collect::<Vec<f64>>();

        // return array
        Array::new(elems, shape)
    }};
}
