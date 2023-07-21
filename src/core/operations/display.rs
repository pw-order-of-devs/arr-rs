use crate::{
    core::prelude::*,
    extensions::prelude::*,
};

impl <T: ArrayElement> std::fmt::Display for Array<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_string(self, f.precision(), f.alternate()))
    }
}

fn build_string<T: ArrayElement>(arr: &Array<T>, precision: Option<usize>, alternate: bool) -> String {
    if arr.is_empty().unwrap_or(true) {
        "[]".to_string()
    } else if arr.len().unwrap_or(0) == 1 {
        format!("[{}]", arr.get_elements().unwrap()[0])
    } else {
        let shape = arr.get_shape().unwrap().reverse_ext();
        let multiply = shape.iter().enumerate()
            .map(|(idx, _)| shape[0..=idx].iter().product())
            .collect::<Vec<usize>>();
        let mut str = vec!["[".repeat(multiply.len())];
        arr.get_elements().unwrap().into_iter().enumerate().for_each(|(idx, elem)|
            str.push(print_subarray(&shape, idx, elem, multiply.clone(), precision, alternate))
        );
        multiply.iter().for_each(|_| str.push("]".to_string()));
        str.join("")
    }
}

fn print_subarray<T: ArrayElement>(shape: &[usize], idx: usize, elem: T, m: Vec<usize>, p: Option<usize>, a: bool) -> String {
    let mut str = vec![];
    if idx == 0 && shape[shape.len() - 1] != 1 {
        str.push(format_with_precision(elem, ", ", p));
    } else if idx == 0 {
        str.push(format_with_precision(elem, "", p));
    } else {
        let separators = m.iter()
            .filter(|&s| idx % s == 0)
            .count();
        str.push("]".repeat(separators));
        if separators > 0 {
            str.push(", ".to_string());
            if a {
                str.push("\n".to_string());
                str.push(" ".repeat(shape.len() - separators));
            }
        }
        str.push("[".repeat(separators));
        str.push(format_with_coma(elem, idx % shape[0] != shape[0] - 1, p));
    }
    str.join("")
}

fn format_with_coma<T: ArrayElement>(elem: T, condition: bool, precision: Option<usize>) -> String {
    if condition {
        format_with_precision(elem, ", ", precision)
    } else {
        format_with_precision(elem, "", precision)
    }
}

fn format_with_precision<T: ArrayElement>(elem: T, suffix: &str, precision: Option<usize>) -> String {
    if precision.is_some() {
        format!("{elem:.p$}{s}", elem = elem, p = precision.unwrap(), s = suffix)
    } else {
        format!("{elem}{suffix}")
    }
}
