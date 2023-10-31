use crate::{
    alphanumeric::prelude::*,
    core::prelude::*,
};
use crate::prelude::Numeric;

impl ArrayElement for char {

    fn zero() -> Self {
        '0'
    }

    fn one() -> Self {
        '1'
    }

    fn is_nan(&self) -> bool {
        false
    }
}

impl Alphanumeric for char {

    fn from_str(str: &str) -> Self {
        str.chars().next().unwrap_or('0')
    }

    fn _append(&self, _: Self) -> Self {
        *self
    }

    fn _multiply(&self, _: usize) -> Self {
        *self
    }

    fn _capitalize(&self) -> Self {
        self.to_string()._capitalize().chars().next().unwrap_or('0')
    }

    fn _lower(&self) -> Self {
        self.to_string()._lower().chars().next().unwrap_or('0')
    }

    fn _upper(&self) -> Self {
        self.to_string()._upper().chars().next().unwrap_or('0')
    }

    fn _swapcase(&self) -> Self {
        self.to_string()._swapcase().chars().next().unwrap_or('0')
    }

    fn _center(&self, _: usize, _: char) -> Self {
        *self
    }

    fn _join(&self, _: Self) -> Self {
        *self
    }

    fn _partition(&self, sep: Self) -> Tuple3<Self, Self, Self> {
        Tuple3(*self, sep, ' ')
    }

    fn _rpartition(&self, sep: Self) -> Tuple3<Self, Self, Self> {
        Tuple3(*self, sep, ' ')
    }

    fn _split(&self, sep: Self, _: Option<usize>) -> List<Self> {
        List(vec![*self, sep, ' '])
    }

    fn _rsplit(&self, sep: Self, _: Option<usize>) -> List<Self> {
        List(vec![' ', sep, *self])
    }

    fn _splitlines(&self, _: bool) -> List<Self> {
        List(vec![*self])
    }

    fn _replace(&self, old: Self, new: Self, _: Option<usize>) -> Self {
        if *self == old { new }
        else { old }
    }

    fn _strip(&self, chars: Self) -> Self {
        if *self == chars { ' ' }
        else { *self }
    }

    fn _ljust(&self, _: usize, _: char) -> Self {
        *self
    }

    fn _lstrip(&self, chars: Self) -> Self {
        if *self == chars { ' ' }
        else { *self }
    }

    fn _rjust(&self, _: usize, _: char) -> Self {
        *self
    }

    fn _rstrip(&self, chars: Self) -> Self {
        if *self == chars { ' ' }
        else { *self }
    }

    fn _equal(&self, other: Self) -> bool {
        *self == other
    }

    fn _not_equal(&self, other: Self) -> bool {
        !self._equal(other)
    }

    fn _greater_equal(&self, other: Self) -> bool {
        *self >= other
    }

    fn _less_equal(&self, other: Self) -> bool {
        *self <= other
    }

    fn _greater(&self, other: Self) -> bool {
        *self > other
    }

    fn _less(&self, other: Self) -> bool {
        *self < other
    }

    fn _count(&self, sub: &str) -> usize {
        (*self == sub.chars().next().unwrap_or(' ')).to_usize()
    }
}
