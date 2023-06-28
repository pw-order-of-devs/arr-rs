use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    types::ArrayElement,
};

impl <N: ArrayElement> IntoIterator for Array<N> {
    type Item = N;
    type IntoIter = std::vec::IntoIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl<'a, N: ArrayElement> IntoIterator for &'a Array<N> {
    type Item = &'a N;
    type IntoIter = std::slice::Iter<'a, N>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.iter()
    }
}

impl <N: ArrayElement> FromIterator<N> for Array<N> {

    fn from_iter<T: IntoIterator<Item=N>>(iter: T) -> Self {
        Self::flat(iter.into_iter().collect()).unwrap()
    }
}
