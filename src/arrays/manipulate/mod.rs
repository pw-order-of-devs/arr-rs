/// axis array functions implementation
pub mod axis;
/// broadcast array functions implementation
pub mod broadcast;
/// split array functions implementation
pub mod split;

use itertools::Itertools;
use crate::arrays::Array;
use crate::ext::vec_ext::VecRemoveAt;
use crate::prelude::ArrayIndexing;
use crate::traits::{
    create::ArrayCreate,
    manipulate::{
        ArrayManipulate,
        broadcast::ArrayBroadcast,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayManipulate<N> for Array<N> {

    fn insert(&self, indices: Vec<usize>, values: &Self, axis: Option<usize>) -> Self {
        let values = self.insert_validate_shapes(&indices, values, axis);
        let params = self.get_insert_parameters(axis);

        let new_shape = self.get_insert_new_shape(&indices, &values, axis);
        Self::insert_validate_indices_values(&indices, &values, &params, axis);

        let indices = self.get_insert_indices(&indices, &values, &params, axis);
        let values = self.get_insert_values(&indices, &values, axis);
        let new_elements = self.get_insert_new_elements(&indices, &values);

        Self::new(new_elements, new_shape)
    }

    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Self {
        if let Some(axis) = axis {
            assert!(axis < self.ndim(), "axis is out of bounds for array");
            let mut sorted_indices = indices;
            sorted_indices.sort();
            sorted_indices.dedup();

            let new_shape = self.shape.iter()
                .enumerate()
                .map(|(i, &dim)| if i == axis { dim.saturating_sub(sorted_indices.len()) } else { dim })
                .collect::<Vec<usize>>();

            let new_elements = self.elements.iter()
                .enumerate()
                .filter_map(|(idx, elem)| {
                    let current_coords = self.index_to_coord(idx);
                    if !sorted_indices.iter().any(|&i| i == current_coords[axis]) { Some(*elem) }
                    else { None }
                }).collect::<Vec<N>>();

            Self::new(new_elements, new_shape)
        } else {
            assert!(indices.iter().all(|&i| i < self.get_elements().len()), "delete index out of the bounds of array");
            let mut new_elements = self.get_elements();
            indices.iter().rev().for_each(|&i| { new_elements.remove(i); });

            Self::flat(new_elements)
        }
    }

    fn append(&self, values: &Self, axis: Option<usize>) -> Self {
        if let Some(axis) = axis {
            assert!(axis < self.ndim(), "axis is out of bounds for array");
            assert_eq!(self.ndim(), values.ndim(), "input array should have the same dimension as the original one");
            assert_eq!(self.get_shape().remove_at(axis), values.get_shape().remove_at(axis), "input array dimensions for the concatenation axis must match exactly");

            let subarrays = if axis == 0 { 1 } else { self.get_shape()[..axis].iter().product::<usize>() };
            let subarray_len = self.get_shape().iter().product::<usize>() / subarrays;
            let indices = (0 .. subarrays).flat_map(|i| vec![subarray_len].iter().cycle()
                .take(values.len() / subarrays)
                .map(|e| e + i * subarray_len)
                .collect::<Vec<usize>>()
            ).collect::<Vec<usize>>();

            let mut new_shape = self.get_shape();
            new_shape[axis] += values.get_shape()[axis];
            let mut new_elements = self.get_elements();

            indices.iter().rev()
                .zip(&values.get_elements())
                .for_each(|(&i, &e)| new_elements.insert(i, e));
            Self::new(new_elements, new_shape)
        } else {
            let mut elements = self.get_elements();
            elements.append(&mut values.get_elements());
            Self::flat(elements)
        }
    }

    fn reshape(&self, shape: Vec<usize>) -> Self {
        assert_eq!(self.elements.len(), shape.iter().product(), "Shape must match values length");
        Self::new(self.elements.clone(), shape)
    }

    fn resize(&self, shape: Vec<usize>) -> Self {
        self.get_elements().into_iter().cycle()
            .take(shape.iter().product::<usize>())
            .collect::<Self>()
            .reshape(shape)
    }

    fn ravel(&self) -> Self {
        Self::new(self.elements.clone(), vec![self.len()])
    }

    fn atleast(&self, n: usize) -> Self {
        assert!(n > 0, "dimension cannot be zero");
        if self.ndim() >= n { return self.clone() }
        let mut new_shape = self.get_shape();
        (0 .. n - self.ndim()).for_each(|_| new_shape.insert(0, 1));
        self.reshape(new_shape)
    }

    fn trim_zeros(&self) -> Self {
        assert_eq!(1, self.ndim(), "trim_zeros is defined only for 1d arrays");
        let new_elements = self.get_elements()
            .into_iter().rev()
            .skip_while(|&e| e == N::ZERO)
            .collect::<Vec<_>>()
            .into_iter().rev()
            .skip_while(|&e| e == N::ZERO)
            .collect::<Vec<_>>();

        Self::flat(new_elements)
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

// ==== insert helpers

struct InsertParameters {
    subarrays: usize,
    chunk_size: usize,
    subarray_len: usize,
}

impl <N: Numeric> Array<N> {

    fn insert_validate_shapes(&self, indices: &Vec<usize>, values: &Self, axis: Option<usize>) -> Self {
        if indices.len() > 1 { let _ = values.broadcast_to(vec![indices.len()]); }
        if axis.is_some() {
            assert!(axis.unwrap() < self.ndim(), "axis is out of bounds for array");
            let _ = values.broadcast_to(self.get_shape());
        }
        if axis.unwrap_or(0) > 0 { values.ravel() } else { values.clone() }
    }

    fn insert_validate_indices_values(indices: &Vec<usize>, values: &Self, params: &InsertParameters, axis: Option<usize>) {
        if axis.is_some() {
            if !(indices.len() == 1 || values.len() == 1 || values.len() == params.chunk_size * params.subarrays) {
                panic!("values and indices don't match for insert")
            }
        } else if !(indices.len() == 1 || values.len() == 1 || indices.len() == values.len()) {
            panic!("values and indices don't match for insert")
        }
    }

    fn get_insert_parameters(&self, axis: Option<usize>) -> InsertParameters {
        if let Some(axis) = axis {
            let subarrays = if axis == 0 { 1 } else { self.get_shape()[..axis].iter().product::<usize>() };
            let chunk_size = self.get_shape()[axis + 1..].iter().product::<usize>();
            InsertParameters { subarrays, chunk_size, subarray_len: self.len() / subarrays }
        } else {
            InsertParameters { subarrays: 1, chunk_size: 1, subarray_len: self.len() }
        }
    }

    fn get_insert_new_shape(&self, indices: &Vec<usize>, values: &Self, axis: Option<usize>) -> Vec<usize> {
        let mut new_shape = self.shape.clone();
        if let Some(axis) = axis {
            if self.ndim() > values.ndim() { new_shape[axis] += indices.len(); }
            else { new_shape[axis] += values.shape[axis % values.ndim()]; }
        } else {
            new_shape = vec![self.elements.len() + std::cmp::max(values.len(), indices.len())];
        }
        new_shape
    }

    fn get_insert_indices(&self, indices: &Vec<usize>, values: &Self, params: &InsertParameters, axis: Option<usize>) -> Vec<usize> {
        if let Some(axis) = axis {
            let mut chunk = self.get_shape();
            chunk.remove(axis);
            let repeat_count =
                if values.ndim() > 1 && axis != 0 { values.get_shape()[0] }
                else if values.len() == 1 { 1 }
                else { values.len() / chunk.iter().product::<usize>() };
            (0 .. params.subarrays).flat_map(|i| indices.iter()
                .flat_map(|&x| std::iter::repeat(x).take(params.chunk_size * repeat_count))
                .map(move |j| j * params.chunk_size + params.subarray_len * i)
            ).collect::<Vec<usize>>()
        }
        else if indices.len() == 1 { vec![indices[0]; values.len()] }
        else { indices.clone() }
    }

    fn get_insert_values(&self, indices: &[usize], values: &Self, axis: Option<usize>) -> Self {
        if axis.is_some() { values.clone().into_iter().cycle().take(values.len() * indices.len()).collect() }
        else if values.len() == 1 { values.broadcast_to(vec![indices.len()]) }
        else { values.clone() }
    }

    fn get_insert_new_elements(&self, indices: &[usize], values: &Self) -> Vec<N> {
        let mut new_elements = self.elements.clone();
        indices.iter().sorted().rev()
            .zip(&values.into_iter().copied().rev().collect::<Vec<N>>())
            .for_each(|(&i, &e)| new_elements.insert(i, e));
        new_elements
    }
}
