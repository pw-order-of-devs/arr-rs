/// Calculate shape for array creation
#[macro_export]
macro_rules! array_parse_shape {
    ($ndim:expr, $str:expr) => {{
        let mut shape = Vec::<usize>::new();
        let mut _string = $str.to_string();
        for i in (0..$ndim).rev() {
            let tmp_str = _string.replace(&format!("{},{}", "]".repeat(i), "[".repeat(i)), "]#[");
            _string = _string[0.._string.find(&"]".repeat(i)).unwrap() + i].to_string();
            shape.push(tmp_str.split("]#[").count());
        };
        shape
    }};
}

/// Parse input string and escape special chars array creation
#[macro_export]
macro_rules! array_parse_input {
    ($str:expr) => {{
        $str.to_string()
            .replace("\", \"", "\",\"")
            .replace("], [", "],[")
            .replace("\\n", "\n")
            .replace("\\r", "\r")
            .replace("\\t", "\t")
            .replace("\\\\", "\\")
            .replace("\\0", "\0")
    }};
}

/// Wrapper for Tuple `array_create!`
#[macro_export]
macro_rules! array_tuple {
    ($tt:ty, $str:expr) => {{
        let string = array_parse_input!($str);
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 2;
        let ndim = if ndim == 0 { 1 } else { ndim };

        let mut tuple_elems = vec![];
        let mut _string = string.clone();
        while _string.contains("(") {
            let start = _string.find("(").unwrap();
            let end = _string.find(")").unwrap();
            tuple_elems.push(_string[start..=end].to_string().replace("\"", ""));
            _string.replace_range(start..=end, "_");
        }

        // get shape
        let shape = array_parse_shape!(ndim, _string);

        // get array elements
        let elems = tuple_elems
            .into_iter()
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();

        // return array
        Array::<$tt>::new(elems, shape)
    }};
}

/// Wrapper for List `array_create!`
#[macro_export]
macro_rules! array_list {
    ($tt:ty, $str:expr) => {{
        let string = array_parse_input!($str);
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 2;
        let ndim = if ndim == 0 { 1 } else { ndim };

        // let mut list_elems = vec![];
        let mut _string = String::new();
        let mut nest_level = 0;
        for c in string.clone().chars() {
            if c == '[' {
                nest_level += 1;
                if nest_level == ndim + 2 { _string.push_str("&["); }
                else { _string.push(c); }
            } else if c == ']' {
                nest_level -= 1;
                _string.push(c);
            } else {
                _string.push(c);
            }
        }

        let mut list_elems = vec![];
        while _string.contains("&") {
            let start = _string.find("&").unwrap();
            let end = _string[start + 1..].find("]").unwrap();
            list_elems.push(_string[start + 2..=start + end].to_string().replace("\"", ""));
            _string.replace_range(start..=start + end + 1, "_");
        }

        // get shape
        let shape = array_parse_shape!(ndim, _string);

        // get array elements
        let elems = list_elems
            .into_iter()
            .map(|e| e.parse().unwrap())
            .collect::<Vec<$tt>>();

        // return array
        Array::<$tt>::new(elems, shape)
    }};
}

/// Wrapper for char `array_create!`
#[macro_export]
macro_rules! array_char {
    ($str:expr) => {{
        let string = array_parse_input!($str);
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 1;
        let ndim = if ndim == 0 { 1 } else { ndim };

        let mut string_elems = vec![];
        let mut _string = string.clone();
        while _string.contains("'") {
            let start = _string.find("'").unwrap();
            let end = _string[start + 1..].find("'").unwrap();
            string_elems.push(_string[start + 1..=start + end].to_string());
            _string.replace_range(start..=start + end + 1, "_");
        }

        // get shape
        let shape = array_parse_shape!(ndim, _string);

        // get array elements
        let elems = string
            .replace("[", "").replace("]", "").replace("'", "")
            .split(",")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<_>>();

        // return array
        Array::<char>::new(elems, shape)
    }};
}

/// Wrapper for String `array_create!`
#[macro_export]
macro_rules! array_string {
    ($str:expr) => {{
        let string = array_parse_input!($str);
        let ndim = string.find(|p| p != '[').unwrap_or(1) - 1;
        let ndim = if ndim == 0 { 1 } else { ndim };

        let mut string_elems = vec![];
        let mut _string = string.clone();
        while _string.contains("\"") {
            let start = _string.find("\"").unwrap();
            let end = _string[start + 1..].find("\"").unwrap();
            string_elems.push(_string[start + 1..=start + end + 1].to_string().replace("\"", ""));
            _string.replace_range(start..=start + end + 1, "_");
        }

        // get shape
        let shape = array_parse_shape!(ndim, string.clone());

        // get array elements
        let elems = string
            .replace("[", "").replace("]", "").replace("\"", "")
            .split(",")
            .map(|e| e.to_string())
            .collect::<Vec<_>>();

        // return array
        Array::<String>::new(elems, shape)
    }};
}
