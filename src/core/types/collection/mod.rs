use std::fmt::{Display, Formatter};

use crate::core::types::ArrayElement;

/// Generic Collection trait for array
pub trait CollectionElement<T: ArrayElement> {}

/// List type for array
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct List<T: ArrayElement>(pub Vec<T>);

impl <T: ArrayElement> CollectionElement<T> for List<T> {}

impl <T: ArrayElement> ArrayElement for List<T> {

    fn zero() -> Self {
        List(vec![])
    }

    fn one() -> Self {
        List(vec![])
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
