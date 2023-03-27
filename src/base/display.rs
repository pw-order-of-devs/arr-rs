use crate::arrays::array::Array;
use crate::base::base_array::ArrayBase;
use crate::base::base_type::Numeric;

impl <N: Numeric> std::fmt::Display for Array<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            write!(f, "{}", build_string(self, true))
        } else {
            write!(f, "{}", build_string(self, false))
        }
    }
}

/// Display trait for numeric Array
pub trait ArrayDisplay<N: Numeric> {}

impl <N: Numeric> ArrayDisplay<N> for Array<N> {}

fn build_string<N: Numeric>(arr: &Array<N>, pretty: bool) -> String {
    if arr.is_empty() {
        return "[]".to_string();
    } else if arr.len() == 1 {
        return format!("[{}]", arr.elements[0]);
    }
    let multiply = arr.shape.iter().enumerate()
        .map(|(idx, _)| arr.shape[0..=idx].iter().product())
        .collect::<Vec<usize>>();
    let mut str = vec!["[".repeat(multiply.len())];
    arr.elements.iter().enumerate().for_each(|(idx, &elem)| {
        if idx == 0 && arr.shape[arr.ndim() - 1] != 1 {
            str.push(format!("{elem}, "));
        } else if idx == 0 {
            str.push(format!("{elem}"));
        } else {
            let separators = multiply.iter()
                .filter(|&s| idx % s == 0)
                .count();
            str.push("]".repeat(separators));
            if separators > 0 {
                str.push(", ".to_string());
                if pretty {
                    str.push("\n".to_string());
                    str.push(" ".repeat(arr.shape.len() - separators));
                }
            }
            str.push("[".repeat(separators));
            if idx % arr.shape[0] != arr.shape[0] - 1 {
                str.push(format!("{elem}, "));
            } else {
                str.push(format!("{elem}"));
            }
        }
    });
    multiply.iter().for_each(|_| str.push("]".to_string()));
    str.join("")
}