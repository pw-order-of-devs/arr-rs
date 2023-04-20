use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayIndexing<N> for Array<N> {

    fn index_at(&self, coords: &[usize]) -> usize {
        assert_eq!(self.shape.len(), coords.len(), "coords length must match array dimension");
        coords.iter().enumerate().for_each(|(i, _)| assert!(coords[i] < self.shape[i], "coord value must match array shape"));

        self.shape.iter().enumerate().rev().fold((0, 1), |(mut index, mut stride), (i, &dim)| {
            index += coords[i] * stride;
            stride *= dim;
            (index, stride)
        }).0
    }

    fn index_to_coord(&self, idx: usize) -> Vec<usize> {
        self.shape.iter().rev().fold((idx, Vec::new()), |(ri, mut coords), &dim| {
            coords.push(ri % dim);
            (ri / dim, coords)
        }).1.into_iter().rev().collect()
    }

    fn at(&self, coords: &[usize]) -> N {
        self.elements[self.index_at(coords)]
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        assert!(range.start <= range.end && range.end <= self.elements.len(), "Slice range out of bounds");
        if self.shape.len() == 1 {
            Self::flat(self.elements[range].into())
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
            (start_index..start_index + items).step_by(stride).for_each(|idx| {
                new_elements.extend_from_slice(&self.elements[idx..idx + stride]);
            });
            Self::new(new_elements, new_shape)
        }
    }
}
