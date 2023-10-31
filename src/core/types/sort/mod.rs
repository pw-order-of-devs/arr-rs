use crate::errors::prelude::*;

/// the kind of sort operation
#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum SortKind {
    /// quicksort
    Quicksort,
    /// mergesort
    Mergesort,
    /// heapsort
    Heapsort,
    /// stable
    Stable,
}

/// `SortKind` trait
pub trait SortKindType {

    /// Parse input to `SortKind` type
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn parse_type(self) -> Result<SortKind, ArrayError>;
}

impl SortKindType for SortKind {

    fn parse_type(self) -> Result<SortKind, ArrayError> {
        Ok(self)
    }
}

impl SortKindType for &str {

    fn parse_type(self) -> Result<SortKind, ArrayError> {
        parse_kind(self.to_lowercase().as_str())
    }
}
impl SortKindType for String {

    fn parse_type(self) -> Result<SortKind, ArrayError> {
        parse_kind(self.to_lowercase().as_str())
    }
}

fn parse_kind(value: &str) -> Result<SortKind, ArrayError> {
    match value {
        "quicksort" => Ok(SortKind::Quicksort),
        "mergesort" => Ok(SortKind::Mergesort),
        "heapsort" => Ok(SortKind::Heapsort),
        "stable" => Ok(SortKind::Stable),
        _ => Err(ArrayError::ParameterError { param: "`kind`", message: "must be one of {`quicksort`, `mergesort`, `heapsort`, `stable`}" })
    }
}
