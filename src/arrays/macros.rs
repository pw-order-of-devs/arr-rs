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

        // get shape
        let ndim = string.find(|p| p != '[').unwrap_or(0);
        let mut str = string[ndim - 1 .. string.len() - (ndim - 1)].to_string();
        let mut shape = (0..ndim).map(|_| {
            let tmp_str = str.replace("[[", "[").replace("]]", "]");
            let parts = tmp_str.split("],[").collect::<Vec<&str>>();
            let shape_part = parts[0].split(',').collect::<Vec<&str>>().len();
            assert!(parts.into_iter().all(|p| p.split(',').collect::<Vec<&str>>().len() == shape_part));
            let parts_to_replace = tmp_str.replace("],[", "],,,[");
            let parts_to_replace = parts_to_replace.split(",,,").collect::<Vec<&str>>();
            parts_to_replace.into_iter().for_each(|f| str = str.replace(f, "0"));
            str = format!("[{str}]");
            shape_part
        }).collect::<Vec<usize>>();
        shape.reverse();

        // get array elements
        let split_items = string.replace("[", "").replace("]", "").replace(" ", "");
        let split = split_items.split(",");
        let elems = split.into_iter().map(|e| f64::from_str(&e).unwrap()).collect::<Vec<f64>>();

        // return array
        Array::new(elems.clone(), shape)
    }};
}
