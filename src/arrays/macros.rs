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
        let string = format!("{:?}", vec![$($x,)*])
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        // get shape
        let ndim = string.chars().take_while(|&c| c == string.chars().next().unwrap_or('[')).count();
        let mut str = string[ndim - 1 .. string.len() - (ndim - 1)].to_string();
        let mut shape = (0..ndim).map(|_| {
            let tmp_str = str.replace("[[", "[").replace("]]", "]").replace("],[", "]#[");
            let parts = tmp_str.split("#").collect::<Vec<&str>>();
            let shape_part = parts[0].split(',').collect::<Vec<&str>>().len();
            assert!(parts.into_iter().all(|p| p.split(',').collect::<Vec<&str>>().len() == shape_part));
            let parts_to_replace = tmp_str.split("#").collect::<Vec<&str>>();
            parts_to_replace.into_iter().for_each(|f| str = str.replace(f, "0"));
            str = format!("[{str}]");
            shape_part
        }).collect::<Vec<usize>>();
        shape.reverse();

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
