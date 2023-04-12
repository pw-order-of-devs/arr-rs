use crate::arrays::Array;
use crate::prelude::ArrayMeta;
use crate::traits::{
    broadcast::ArrayBroadcast,
    create::ArrayCreate,
    types::{
        numeric::Numeric,
        tuple_numeric::Tuple2,
    },
};

impl <N: Numeric> ArrayBroadcast<N> for Array<N> {

    fn broadcast(&self, other: &Self) -> Array<Tuple2<N>> {
        self.validate_shapes(other.get_shape());
        let final_shape = self.broadcast_shape(other.get_shape());

        let inner_arrays_self = self.extract_inner_arrays();
        let inner_arrays_other = other.extract_inner_arrays();

        let output_elements = inner_arrays_self.iter().cycle()
            .zip(inner_arrays_other.iter().cycle())
            .flat_map(|(inner_self, inner_other)| {
                inner_self.iter().cycle()
                    .zip(inner_other.iter().cycle())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(&a, &b)| Tuple2(a, b))
            })
            .take(final_shape.iter().product())
            .collect::<Vec<_>>();

        Array::new(output_elements, final_shape)
    }

    fn broadcast_to(&self, shape: Vec<usize>) -> Array<N> {
        self.validate_shapes(shape.clone());

        let output_elements: Vec<N> = self.elements
            .chunks_exact(self.shape[self.shape.len() - 1])
            .flat_map(|inner| {
                let extended_inner = inner.iter()
                    .cycle()
                    .take(shape[shape.len() - 1])
                    .copied()
                    .collect::<Vec<N>>();
                extended_inner.into_iter()
            })
            .cycle()
            .take(shape.iter().product())
            .collect();

        Array::new(output_elements, shape)
    }

    fn broadcast_arrays(arrays: Vec<Self>) -> Vec<Self> {
        let mut max_shape = arrays[0].get_shape();
        arrays[1..].iter().for_each(|array|
            max_shape = array.broadcast_shape(max_shape.clone())
        );

        arrays.iter()
            .map(|array| array.broadcast_to(max_shape.clone()))
            .collect::<Vec<Array<N>>>()
    }
}

impl <N: Numeric> Array<N> {

    fn validate_shapes(&self, shape: Vec<usize>) {
        if self.shape.iter()
            .zip(shape.iter())
            .take(self.shape.len().max(shape.len()))
            .rev()
            .any(|(&dim1, &dim2)| dim1 != dim2 && dim1 != 1 && dim2 != 1 || dim1 == 0 || dim2 == 0) {
            panic!("incompatible shapes for broadcasting");
        }
    }

    fn broadcast_shape(&self, shape: Vec<usize>) -> Vec<usize> {
        let max_dim = self.shape.len().max(shape.len());
        let shape1_padded = self.shape.iter().rev()
            .copied().chain(std::iter::repeat(1))
            .take(max_dim);
        let shape2_padded = shape.iter().rev()
            .copied().chain(std::iter::repeat(1))
            .take(max_dim);

        shape1_padded
            .zip(shape2_padded.into_iter())
            .map(|(dim1, dim2)| {
                if dim1 == 1 { dim2 }
                else if dim2 == 1 || dim1 == dim2 { dim1 }
                else { panic!("incompatible shapes for broadcasting"); }
            })
            .collect()
    }

    fn extract_inner_arrays(&self) -> Vec<Vec<N>> {
        match self.shape.len() {
            1 => vec![self.elements.clone()],
            _ => self.elements
                .chunks_exact(*self.shape.last().unwrap())
                .map(Vec::from)
                .collect(),
        }
    }
}
