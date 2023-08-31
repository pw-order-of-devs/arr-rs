/// char definition
pub mod char;

/// String definition
pub mod string;

use crate::core::types::{
    ArrayElement,
    collection::List,
    tuple::tuple3::Tuple3,
};

/// Alphanumeric trait for array
pub trait Alphanumeric: ArrayElement {

    /// parse from &str
    fn from_str(str: &str) -> Self;

    /// append string with another
    fn _append(&self, other: Self) -> Self;

    /// multiply string n-times
    fn _multiply(&self, n: usize) -> Self;

    /// capitalize string
    fn _capitalize(&self) -> Self;

    /// lower case string
    fn _lower(&self) -> Self;

    /// upper case string
    fn _upper(&self) -> Self;

    /// swap case in string
    fn _swapcase(&self) -> Self;

    /// center string elements
    fn _center(&self, width: usize, fill_char: char) -> Self;

    /// join string by separator
    fn _join(&self, sep: Self) -> Self;

    /// partition string by first occurrence of separator
    fn _partition(&self, sep: Self) -> Tuple3<Self, Self, Self>;

    /// partition string by last occurrence of separator
    fn _rpartition(&self, sep: Self) -> Tuple3<Self, Self, Self>;

    /// split string by separator
    fn _split(&self, sep: Self, max_split: Option<usize>) -> List<Self>;

    /// split string by separator from right
    fn _rsplit(&self, sep: Self, max_split: Option<usize>) -> List<Self>;

    /// split string by line break character
    fn _splitlines(&self, keep_ends: bool) -> List<Self>;

    /// replace <old> string with <new> <count> times
    fn _replace(&self, old: Self, new: Self, count: Option<usize>) -> Self;

    /// strips string elements
    fn _strip(&self, chars: Self) -> Self;

    /// left-justifies string elements
    fn _ljust(&self, width: usize, fill_char: char) -> Self;

    /// left-strips string elements
    fn _lstrip(&self, chars: Self) -> Self;

    /// right-justifies string elements
    fn _rjust(&self, width: usize, fill_char: char) -> Self;

    /// right-strips string elements
    fn _rstrip(&self, chars: Self) -> Self;

    /// is equal to
    fn _equal(&self, other: Self) -> bool;

    /// is not equal to
    fn _not_equal(&self, other: Self) -> bool;

    /// is greater or equal to
    fn _greater_equal(&self, other: Self) -> bool;

    /// is less or equal to
    fn _less_equal(&self, other: Self) -> bool;

    /// is greater than
    fn _greater(&self, other: Self) -> bool;

    /// is less than
    fn _less(&self, other: Self) -> bool;

    /// counts non-overlapping occurrences of substring
    fn _count(&self, sub: &str) -> usize;
}
