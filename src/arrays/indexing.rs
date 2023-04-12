use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayIndexing<N> for Array<N> {

    fn index_at(&self, coords: &[usize]) -> usize {
        assert_eq!(self.shape.len(), coords.len(), "coords length must match array dimension");
        for (i, _) in coords.iter().enumerate() { assert!(coords[i] < self.shape[i], "coord value must match array shape"); }
        let mut index = 0;
        let mut stride = 1;
        for i in (0..self.shape.len()).rev() {
            index += coords[i] * stride;
            stride *= self.shape[i];
        }
        index
    }

    fn at(&self, coords: &[usize]) -> N {
        self.elements[self.index_at(coords)]
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        assert!(range.start <= range.end && range.end <= self.elements.len(), "Slice range out of bounds");
        if self.shape.len() == 1 {
            Array::flat(self.elements[range].into())
        } else if range.len() >= self.shape[0] {
            self.clone()
        } else {
            let new_shape =
                if range.len() > 1 { vec![range.len()].into_iter().chain(self.shape[1..].iter().cloned()).collect() }
                else { self.shape[1..].to_vec() };

            let items: usize = new_shape.iter().product();
            let stride = items / new_shape[0];
            let start_index = new_shape[0] * range.start;

            let mut new_elements = Vec::with_capacity(items);
            for idx in (start_index..start_index + items).step_by(stride) {
                new_elements.extend_from_slice(&self.elements[idx..idx + stride]);
            }
            Array::new(new_elements, new_shape)
        }
    }
}
