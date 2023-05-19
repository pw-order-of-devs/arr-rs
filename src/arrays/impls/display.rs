use crate::arrays::Array;
use crate::traits::{
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> std::fmt::Display for Array<N> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_string(self, f.precision(), f.alternate()))
    }
}

fn build_string<N: Numeric>(arr: &Array<N>, precision: Option<usize>, alternate: bool) -> String {
    if arr.is_empty().unwrap_or(true) {
        "[]".to_string()
    } else if arr.len().unwrap_or(0) == 1 {
        format!("[{}]", arr.get_elements().unwrap()[0])
    } else {
        let mut shape = arr.get_shape().unwrap();
        shape.reverse();
        let multiply = shape.iter().enumerate()
            .map(|(idx, _)| shape[0..=idx].iter().product())
            .collect::<Vec<usize>>();
        let mut str = vec!["[".repeat(multiply.len())];
        arr.get_elements().unwrap().iter().enumerate().for_each(|(idx, &elem)|
            str.push(print_subarray(&shape, idx, elem, multiply.clone(), precision, alternate))
        );
        multiply.iter().for_each(|_| str.push("]".to_string()));
        str.join("")
    }
}

fn print_subarray<N: Numeric>(shape: &[usize], idx: usize, elem: N, m: Vec<usize>, p: Option<usize>, a: bool) -> String {
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

fn format_with_coma<N: Numeric>(elem: N, condition: bool, precision: Option<usize>) -> String {
    if condition {
        format_with_precision(elem, ", ", precision)
    } else {
        format_with_precision(elem, "", precision)
    }
}

fn format_with_precision<N: Numeric>(elem: N, suffix: &str, precision: Option<usize>) -> String {
    if precision.is_some() {
        format!("{elem:.p$}{s}", elem = elem, p = precision.unwrap(), s = suffix)
    } else {
        format!("{elem}{suffix}")
    }
}
