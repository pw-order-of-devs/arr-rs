use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::core::types::ArrayElement;

/// Generic Collection trait for array
pub trait CollectionElement<T: ArrayElement> {}

/// List type for array
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct List<T: ArrayElement>(pub Vec<T>);

impl<T: ArrayElement + FromStr> FromStr for List<T>
    where <T as FromStr>::Err: std::fmt::Debug, {
    type Err = ParseListError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .trim_start_matches('(')
            .trim_end_matches(')')
            .replace(", ", ",");
        let mut items = vec![];
        for item in s.split(',') {
            let item = T::from_str(item);
            if item.is_err() {
                return Err(ParseListError::Parse("error parsing list value"))
            }
            items.push(item.unwrap())
        }

        Ok(List(items))
    }
}

impl <T: ArrayElement> CollectionElement<T> for List<T> {}

impl <T: ArrayElement> ArrayElement for List<T> {

    fn zero() -> Self {
        List(vec![])
    }

    fn one() -> Self {
        List(vec![])
    }

    fn is_nan(&self) -> bool {
        self.0.iter().any(|i| i.is_nan())
    }
}

impl <T: ArrayElement> Display for List<T> {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(", "))
    }
}

/// Error definition for list parsing
#[derive(Debug)]
pub enum ParseListError {
    /// Error definition for list parsing - Parse error
    Parse(&'static str),
    /// Error definition for list parsing - Format error
    Format,
}
