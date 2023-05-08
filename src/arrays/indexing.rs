use crate::arrays::Array;
use crate::prelude::{ArrayError, ArrayMeta};
use crate::traits::{
    create::ArrayCreate,
    indexing::ArrayIndexing,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayIndexing<N> for Array<N> {

    fn index_at(&self, coords: &[usize]) -> Result<usize, ArrayError> {
        if self.shape.len() != coords.len() {
            Err(ArrayError::ParameterError { param: "coords", message: "length must match array dimension", })
        } else if coords.iter().enumerate().any(|(i, _)| coords[i] >= self.shape[i]) {
            Err(ArrayError::ParameterError { param: "coords", message: "value must match array shape", })
        } else {
            let result = self.shape.iter().enumerate().rev().fold((0, 1), |(mut index, mut stride), (i, &dim)| {
                index += coords[i] * stride;
                stride *= dim;
                (index, stride)
            }).0;
            Ok(result)
        }
    }

    fn index_to_coord(&self, idx: usize) -> Result<Vec<usize>, ArrayError> {
        if idx >= self.len() {
            Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", })
        } else {
            let result = self.shape.iter().rev().fold((idx, Vec::new()), |(ri, mut coords), &dim| {
                coords.push(ri % dim);
                (ri / dim, coords)
            }).1.into_iter().rev().collect();
            Ok(result)
        }
    }

    fn at(&self, coords: &[usize]) -> Result<N, ArrayError> {
        match self.index_at(coords) {
            Ok(idx) => Ok(self.elements[idx]),
            Err(e) => Err(e),
        }
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Result<Self, ArrayError> {
        if !(range.start <= range.end && range.end <= self.elements.len()) {
            return Err(ArrayError::OutOfBounds { value: "slice range" })
        }

        if self.shape.len() == 1 {
            Ok(Self::flat(self.elements[range].into()))
        } else if range.len() >= self.shape[0] {
            Ok(self.clone())
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
