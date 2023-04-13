use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    manipulate::ArrayManipulate,
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayManipulate<N> for Array<N> {

    fn reshape(&self, shape: Vec<usize>) -> Self {
        assert_eq!(self.elements.len(), shape.iter().product(), "Shape must match values length");
        Array::new(self.elements.clone(), shape)
    }

    fn ravel(&self) -> Self {
        Array::new(self.elements.clone(), vec![self.len()])
    }

    fn transpose(&self) -> Self {
        let mut new_elements = vec![N::ZERO; self.elements.len()];
        let mut new_shape = self.shape.clone();
        new_shape.reverse();

        fn transpose_recursive<N: Numeric>(
            input: &[N],
            input_shape: &[usize],
            output: &mut [N],
            output_shape: &[usize],
            current_indices: &mut [usize],
            current_dim: usize,
        ) {
            if current_dim < input_shape.len() - 1 {
                for i in 0..input_shape[current_dim] {
                    current_indices[current_dim] = i;
                    transpose_recursive(input, input_shape, output, output_shape, current_indices, current_dim + 1);
                }
            } else {
                for i in 0..input_shape[current_dim] {
                    current_indices[current_dim] = i;
                    let input_index = input_shape.iter().enumerate().fold(0, |acc, (dim, size)| {
                        acc * size + current_indices[dim]
                    });
                    let output_indices = current_indices.iter().rev().cloned().collect::<Vec<usize>>();
                    let output_index = output_shape.iter().enumerate().fold(0, |acc, (dim, size)| {
                        acc * size + output_indices[dim]
                    });
                    output[output_index] = input[input_index];
                }
            }
        }

        transpose_recursive(
            &self.elements,
            &self.shape,
            &mut new_elements,
            &new_shape,
            &mut vec![0; self.shape.len()],
            0,
        );

        Self::new(new_elements, new_shape)
    }

    fn for_each<F: FnMut(&N)>(&self, f: F) {
        self.elements.iter()
            .for_each(f)
    }

    fn for_each_e<F: FnMut(usize, &N)>(&self, mut f: F) {
        self.elements.iter().enumerate()
            .for_each(|(idx, item)| f(idx, item))
    }

    fn map<F: FnMut(&N) -> N>(&self, f: F) -> Self {
        self.elements.iter()
            .map(f)
            .collect()
    }

    fn map_e<F: FnMut(usize, &N) -> N>(&self, mut f: F) -> Self {
        self.elements.iter().enumerate()
            .map(|(idx, item)| f(idx, item))
            .collect()
    }

    fn filter<F: FnMut(&N) -> bool>(&self, mut f: F) -> Self {
        self.elements.clone().into_iter()
            .filter(|item| f(item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_e<F: FnMut(usize, &N) -> bool>(&self, mut f: F) -> Self {
        self.elements.clone().into_iter().enumerate()
            .filter(|(idx, item)| f(*idx, item))
            .map(|i| i.1)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map<F: FnMut(&N) -> Option<N>>(&self, f: F) -> Self {
        self.elements.clone().iter()
            .filter_map(f)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map_e<F: FnMut(usize, &N) -> Option<N>>(&self, mut f: F) -> Self {
        self.elements.clone().iter().enumerate()
            .filter_map(|(idx, item)| f(idx, item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn fold<F: FnMut(&N, &N) -> N>(&self, init: N, mut f: F) -> N {
        self.elements.iter().fold(init, |a, b| f(&a, b))
    }
}
