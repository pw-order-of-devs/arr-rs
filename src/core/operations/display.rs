use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};

impl <T: ArrayElement> std::fmt::Display for Array<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", build_string(self, f.precision(), f.alternate(), 1))
    }
}

fn build_string<T: ArrayElement>(arr: &Array<T>, precision: Option<usize>, alternate: bool, prefix: usize) -> String {
    if arr.is_empty().unwrap_or(true) {
        "[]".to_string()
    } else if arr.len().unwrap_or(0) == 1 {
        format!("[{}]", arr.get_elements().unwrap()[0])
    } else if arr.ndim().unwrap_or(0) == 1 {
        let elements = arr.get_elements().unwrap().into_iter()
            .map(|e| format_with_precision(e, precision))
            .collect::<Vec<String>>()
            .join(", ");
        format!("[{elements}]")
    } else {
        let arrays = arr.split_axis(0).unwrap_or(vec![]).into_iter()
            .map(|arr| arr.reshape(&arr.get_shape().unwrap().remove_at(0)).unwrap())
            .map(|arr| build_string(&arr, precision, alternate, prefix + 1))
            .collect::<Vec<String>>();
        let separator_alt = format!(",\n{}", " ".repeat(prefix));
        format!("[{}]", arrays.join(if alternate { &separator_alt } else { ", " }))
    }
}

fn format_with_precision<T: ArrayElement>(elem: T, precision: Option<usize>) -> String {
    if precision.is_some() {
        format!("{elem:.p$}", elem = elem, p = precision.unwrap())
    } else {
        format!("{elem}")
    }
}

/// Wrapper for Result<Array<T>, ArrayError> for implementation of Display trait
#[derive(Clone, Debug)]
pub struct PrintableResult<T: ArrayElement> {
    /// inner field - result to print
    pub result: Result<Array<T>, ArrayError>,
}

impl <T: ArrayElement> std::fmt::Display for PrintableResult<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self.result.clone();
        if self.result.is_err() {
            write!(f, "Err({})", result.unwrap_err())
        } else {
            write!(f, "Ok({})", build_string(&result.unwrap(), f.precision(), f.alternate(), 1))
        }
    }
}
