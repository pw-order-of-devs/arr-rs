use crate::arrays::Array;
use crate::prelude::{ArrayManipulate, ArrayMeta};
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
            .flat_map(|(inner_self, inner_other)| match (inner_self.len(), inner_other.len()) {
                (1, _) => inner_self.iter().cycle()
                    .zip(inner_other.iter())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(&a, &b)| Tuple2(a, b))
                    .collect::<Vec<_>>(),
                (_, 1) => inner_self.iter()
                    .zip(inner_other.iter().cycle())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(&a, &b)| Tuple2(a, b))
                    .collect::<Vec<_>>(),
                _ => inner_self.iter().cycle()
                    .zip(inner_other.iter().cycle())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(&a, &b)| Tuple2(a, b))
                    .collect::<Vec<_>>(),
            })
            .take(final_shape.iter().product())
            .collect::<Vec<_>>();

        Array::new(output_elements, final_shape)
    }

    fn broadcast_to(&self, shape: Vec<usize>) -> Array<N> {
        self.validate_shapes(shape.clone());
        if self.get_shape().iter().product::<usize>() == shape.iter().product() {
            return self.reshape(shape);
        }

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
        let shapes = arrays.iter()
            .map(|array| array.get_shape())
            .collect::<Vec<_>>();
        let common_shape = Self::common_broadcast_shape(&shapes);

        arrays.iter()
            .map(|array| array.broadcast_to(common_shape.clone()))
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

    fn common_broadcast_shape(shapes: &[Vec<usize>]) -> Vec<usize> {
        let max_dim = shapes.iter()
            .map(|shape| shape.len())
            .max().unwrap_or(0);

        let shapes_padded: Vec<_> = shapes
            .iter()
            .map(|shape| shape.iter().rev().copied()
                .chain(std::iter::repeat(1))
                .take(max_dim)
                .collect::<Vec<_>>()
            )
            .collect();

        let common_shape: Vec<usize> = (0..max_dim)
            .map(|dim_idx| shapes_padded.iter()
                .map(|shape| shape[dim_idx])
                .max().unwrap_or(1)
            )
            .collect();

        let is_compatible = shapes_padded.iter()
            .all(|shape| common_shape.iter().enumerate()
                .all(|(dim_idx, &common_dim)| {
                    let dim = shape[dim_idx];
                    dim == common_dim || dim == 1 || common_dim == 1
                })
            );

        if is_compatible { common_shape.into_iter().rev().collect() }
        else { panic!("incompatible shapes for broadcasting"); }
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
