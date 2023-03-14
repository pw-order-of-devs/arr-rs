use crate::arrays::array::Array;
use crate::base::base_array::ArrayBase;
use crate::base::base_type::Numeric;

impl <N: Numeric> IntoIterator for Array<N> {
    type Item = N;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.elements.into_iter()
    }
}

impl <N: Numeric> FromIterator<N> for Array<N> {
    fn from_iter<T: IntoIterator<Item=N>>(iter: T) -> Self {
        let elems: Vec<N> = iter.into_iter().collect();
        Array::new(elems.clone(), vec![elems.len()])
    }
}
