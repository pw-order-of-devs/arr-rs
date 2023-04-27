use std::collections::HashSet;
use itertools::Itertools;
use crate::arrays::Array;
use crate::prelude::ArrayManipulate;
use crate::traits::{
    manipulate::axis::ArrayAxis,
    meta::ArrayMeta,
    create::ArrayCreate,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayAxis<N> for Array<N> {

    fn transpose(&self, axes: Option<Vec<isize>>) -> Self {

        fn transpose_recursive<N: Numeric>(
            input: &[N], input_shape: &[usize],
            output: &mut [N], output_shape: &[usize],
            current_indices: &mut [usize], current_dim: usize,
            axes: &Option<Vec<usize>>) {
            if current_dim < input_shape.len() - 1 {
                (0 .. input_shape[current_dim]).for_each(|i| {
                    current_indices[current_dim] = i;
                    transpose_recursive(input, input_shape, output, output_shape, current_indices, current_dim + 1, axes);
                });
            } else {
                (0 .. input_shape[current_dim]).for_each(|i| {
                    current_indices[current_dim] = i;
                    let input_index = input_shape.iter().enumerate().fold(0, |acc, (dim, size)| { acc * size + current_indices[dim] });
                    let output_indices = match axes {
                        Some(ref axes) => axes.iter().map(|&ax| current_indices[ax]).collect::<Vec<usize>>(),
                        None => current_indices.iter().rev().cloned().collect::<Vec<usize>>(),
                    };
                    let output_index = output_shape.iter().enumerate().fold(0, |acc, (dim, size)| { acc * size + output_indices[dim] });
                    output[output_index] = input[input_index];
                });
            }
        }

        let axes = axes.map(|axes| axes.iter()
            .map(|i| self.normalize_axis(*i, None))
            .collect::<Vec<usize>>());
        let mut new_elements = vec![N::ZERO; self.elements.len()];
        let new_shape: Vec<usize> = match axes.clone() {
            Some(axes) => axes.into_iter().map(|ax| self.shape[ax]).collect(),
            None => self.shape.clone().into_iter().rev().collect(),
        };

        transpose_recursive(
            &self.elements, &self.shape,
            &mut new_elements, &new_shape,
            &mut vec![0; self.shape.len()], 0,
            &axes
        );

        Self::new(new_elements, new_shape)
    }

    fn moveaxis(&self, source: Vec<isize>, destination: Vec<isize>) -> Self {
        assert_eq!(source.len(), destination.len(), "`source` and `destination` should have the same length");
        let source = source.iter().map(|i| self.normalize_axis(*i, None)).collect::<Vec<usize>>();
        let destination = destination.iter().map(|i| self.normalize_axis(*i, None)).collect::<Vec<usize>>();
        assert_eq!(HashSet::<usize>::from_iter(source.iter().cloned()).len(), source.len(), "`source` axes should be unique");
        assert_eq!(HashSet::<usize>::from_iter(destination.iter().cloned()).len(), destination.len(), "`destination` axes should be unique");

        let mut order = (0 .. self.ndim())
            .filter(|f| !source.contains(f))
            .collect::<Vec<usize>>();

        destination.into_iter()
            .zip(source.into_iter())
            .sorted()
            .for_each(|(d, s)| order.insert(d.min(order.len()), s));

        let order = order.iter().map(|i| *i as isize).collect();
        self.transpose(Some(order))
    }

    fn rollaxis(&self, axis: isize, start: Option<isize>) -> Self {
        let axis = self.normalize_axis(axis, None);
        let start = if let Some(start) = start { self.normalize_axis(start, None) } else { 0 };

        let mut new_axes = (0 .. self.ndim()).collect::<Vec<usize>>();
        let axis_to_move = new_axes.remove(axis);
        new_axes.insert(start, axis_to_move);

        self.transpose(Some(new_axes.iter().map(|&i| i as isize).collect()))
    }

    fn swapaxes(&self, axis_1: isize, axis_2: isize) -> Self {
        let axis_1 = self.normalize_axis(axis_1, None);
        let axis_2 = self.normalize_axis(axis_2, None);

        let mut new_axes = (0 .. self.ndim()).collect::<Vec<usize>>();
        new_axes.swap(axis_1, axis_2);

        self.transpose(Some(new_axes.iter().map(|&i| i as isize).collect()))
    }

    fn expand_dims(&self, axes: Vec<isize>) -> Self {
        let axes = axes.iter()
            .map(|&i| self.normalize_axis(i, Some(self.ndim() + axes.len())))
            .sorted()
            .collect::<Vec<usize>>();
        let mut new_shape = self.get_shape();

        for item in axes { new_shape.insert(item, 1) }
        self.reshape(new_shape)
    }

    fn squeeze(&self, axes: Option<Vec<isize>>) -> Self {
        if let Some(axes) = axes {
            let axes = axes.iter()
                .map(|&i| self.normalize_axis(i, None))
                .sorted()
                .rev()
                .collect::<Vec<usize>>();
            let mut new_shape = self.get_shape();

            for item in axes {
                assert_eq!(1, self.get_shape()[item], "cannot select an axis to squeeze out which has size not equal to one");
                new_shape.remove(item);
            }
            self.reshape(new_shape)
        }
        else { self.reshape(self.get_shape().into_iter().filter(|&i| i != 1).collect()) }
    }
}

impl <N: Numeric> Array<N> {

    fn normalize_axis(&self, axis: isize, ndim: Option<usize>) -> usize {
        if axis < 0 { (axis + ndim.unwrap_or(self.ndim()) as isize) as usize }
        else { axis as usize }
    }
}
