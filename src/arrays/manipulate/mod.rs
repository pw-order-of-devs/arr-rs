/// axis array functions implementation
pub mod axis;
/// broadcast array functions implementation
pub mod broadcast;
/// split array functions implementation
pub mod split;
/// stack array functions implementation
pub mod stack;

use std::cmp::Ordering;
use itertools::Itertools;
use crate::arrays::Array;
use crate::ext::vec_ext::{VecInsertAt, VecRemoveAt};
use crate::traits::{
    create::ArrayCreate,
    errors::ArrayError,
    indexing::ArrayIndexing,
    manipulate::{
        ArrayManipulate,
        axis::ArrayAxis,
        broadcast::ArrayBroadcast,
        split::ArraySplit,
    },
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayManipulate<N> for Array<N> {

    fn insert(&self, indices: Vec<usize>, values: &Self, axis: Option<usize>) -> Result<Self, ArrayError> {
        if axis.is_some() && axis.unwrap() >= self.ndim() { return Err(ArrayError::AxisOutOfBounds) }

        if indices.len() > 1 {
            let validate_result = values.broadcast_to(vec![indices.len()]);
            if validate_result.is_err() { return Err(validate_result.err().unwrap()) }
        }
        if axis.is_some() {
            let validate_result = values.broadcast_to(self.get_shape());
            if validate_result.is_err() { return Err(validate_result.err().unwrap()) }
        }

        let values = if axis.unwrap_or(0) > 0 { values.ravel() } else { values.clone() };
        let (subarrays, chunk_size, subarray_len) =
            if let Some(axis) = axis {
                let subarrays = if axis == 0 { 1 } else { self.get_shape()[..axis].iter().product::<usize>() };
                let chunk_size = self.get_shape()[axis + 1..].iter().product::<usize>();
                (subarrays, chunk_size, self.len() / subarrays)
            } else { (1, 1, self.len()) };

        let mut new_shape = self.shape.clone();
        if let Some(axis) = axis {
            if self.ndim() > values.ndim() { new_shape[axis] += indices.len(); }
            else { new_shape[axis] += values.shape[axis % values.ndim()]; }
        } else {
            new_shape = vec![self.elements.len() + std::cmp::max(values.len(), indices.len())];
        }

        let axis_some_cond = axis.is_some() && !(indices.len() == 1 || values.len() == 1 || values.len() == chunk_size * subarrays);
        let axis_none_cond = axis.is_none() && !(indices.len() == 1 || values.len() == 1 || indices.len() == values.len());
        if axis_none_cond || axis_some_cond { return Err(ArrayError::ParameterError { param: "values|indices", message: "don't match for insert", }) }

        let indices =
            if let Some(axis) = axis {
                let mut chunk = self.get_shape();
                chunk.remove(axis);
                let repeat_count =
                    if values.ndim() > 1 && axis != 0 { values.get_shape()[0] }
                    else if values.len() == 1 { 1 }
                    else { values.len() / chunk.iter().product::<usize>() };
                (0 .. subarrays).flat_map(|i| indices.iter()
                    .flat_map(|&x| std::iter::repeat(x).take(chunk_size * repeat_count))
                    .map(move |j| j * chunk_size + subarray_len * i)
                ).collect::<Vec<usize>>()
            }
            else if indices.len() == 1 { vec![indices[0]; values.len()] }
            else { indices };

        let values =
            if axis.is_some() { Ok(values.clone().into_iter().cycle().take(values.len() * indices.len()).collect()) }
            else if values.len() == 1 { values.broadcast_to(vec![indices.len()]) }
            else { Ok(values) };
        if values.is_err() { return Err(values.err().unwrap()) }

        let mut new_elements = self.elements.clone();
        indices.iter().sorted().rev()
            .zip(&values.unwrap().get_elements().iter().copied().rev().collect::<Vec<N>>())
            .for_each(|(&i, &e)| new_elements.insert(i, e));

        Self::new(new_elements, new_shape)
    }

    fn delete(&self, indices: Vec<usize>, axis: Option<usize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            if axis >= self.ndim() { return Err(ArrayError::AxisOutOfBounds) }
            let mut sorted_indices = indices;
            sorted_indices.sort();
            sorted_indices.dedup();

            let new_shape = self.shape.iter()
                .enumerate()
                .map(|(i, &dim)| if i == axis { dim.saturating_sub(sorted_indices.len()) } else { dim })
                .collect::<Vec<usize>>();

            if self.elements.iter()
                .enumerate()
                .any(|(idx, _)| self.index_to_coord(idx).is_err()) {
                return Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", });
            }

            let new_elements = self.elements.iter()
                .enumerate()
                .filter_map(|(idx, elem)| {
                    let current_coords = self.index_to_coord(idx).unwrap();
                    if !sorted_indices.iter().any(|&i| i == current_coords[axis]) { Some(*elem) }
                    else { None }
                }).collect::<Vec<N>>();

            Self::new(new_elements, new_shape)
        } else {
            if indices.iter().any(|&i| i >= self.get_elements().len()) { return Err(ArrayError::OutOfBounds { value: "index", }) }
            let mut new_elements = self.get_elements();
            indices.iter().rev().for_each(|&i| { new_elements.remove(i); });

            Ok(Self::flat(new_elements))
        }
    }

    fn append(&self, values: &Self, axis: Option<usize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            if axis >= self.ndim() {
                return Err(ArrayError::AxisOutOfBounds)
            } else if self.ndim() != values.ndim() {
                return Err(ArrayError::ParameterError { param: "values", message: "input array should have the same dimension as the original one", })
            } else if self.get_shape().remove_at(axis) != values.get_shape().remove_at(axis) {
                return Err(ArrayError::ParameterError { param: "axis", message: "input array dimensions for the concatenation axis must match exactly", })
            }

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
                .zip(&values.get_elements().iter().rev().collect::<Vec<&N>>())
                .for_each(|(&i, &&e)| new_elements.insert(i, e));
            Self::new(new_elements, new_shape)
        } else {
            let mut elements = self.get_elements();
            elements.append(&mut values.get_elements());
            Ok(Self::flat(elements))
        }
    }

    fn reshape(&self, shape: Vec<usize>) -> Result<Self, ArrayError> {
        if self.elements.len() != shape.iter().product() {
            Err(ArrayError::ShapeMustMatchValuesLength)
        } else {
            Self::new(self.elements.clone(), shape)
        }
    }

    fn resize(&self, shape: Vec<usize>) -> Result<Self, ArrayError> {
        self.get_elements().into_iter().cycle()
            .take(shape.iter().product::<usize>())
            .collect::<Self>()
            .reshape(shape)
    }

    fn unique(&self, axis: Option<usize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let split = self.split(self.shape[axis], Some(axis))?;
            let mut parts = split.into_iter()
                .sorted_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .collect::<Vec<Self>>();
            parts.dedup();
            let flat_parts = parts.clone().into_iter().flatten().collect::<Vec<N>>();
            if flat_parts.len() == self.len() { Ok(self.clone()) }
            else {
                let new_shape = self.get_shape()
                    .remove_at(axis)
                    .insert_at(axis, parts.len());
                let result = Array::new(flat_parts, new_shape);
                if !(axis > 0 && axis < self.ndim() - 1) { result }
                else { match result {
                    Ok(res) => res.rollaxis(axis as isize, None),
                    Err(e) => Err(e)
                } }
            }
        } else {
            let mut new_elements = self.get_elements().into_iter()
                .sorted_by(|&a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
                .collect::<Vec<N>>();
            new_elements.dedup();
            Ok(Array::flat(new_elements))
        }
    }

    fn ravel(&self) -> Self {
        Self::flat(self.elements.clone())
    }

    fn atleast(&self, n: usize) -> Result<Self, ArrayError> {
        match n {
            0 => Ok(self.clone()),
            1 => Self::atleast_1d(self),
            2 => Self::atleast_2d(self),
            3 => Self::atleast_3d(self),
            _ => Err(ArrayError::UnsupportedDimension { fun: "atleast", supported: "[1D, 2D, 3D]", }),
        }
    }

    fn trim_zeros(&self) -> Result<Self, ArrayError> {
        if self.ndim() != 1 { return Err(ArrayError::UnsupportedDimension { fun: "trim_zeros", supported: "1D", }) }
        let new_elements = self.get_elements()
            .into_iter().rev()
            .skip_while(|&e| e == N::ZERO)
            .collect::<Vec<_>>()
            .into_iter().rev()
            .skip_while(|&e| e == N::ZERO)
            .collect::<Vec<_>>();

        Ok(Self::flat(new_elements))
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

impl <N: Numeric> Array<N> {

    fn atleast_1d(&self) -> Result<Self, ArrayError> {
        if !self.ndim() >= 1 { Ok(self.clone()) }
        else { self.reshape(vec![1]) }
    }

    fn atleast_2d(&self) -> Result<Self, ArrayError> {
        if self.ndim() >= 2 { Ok(self.clone()) }
        else {
            match self.ndim() {
                0 => self.reshape(vec![1, 1]),
                1 => self.reshape(vec![1, self.get_shape()[0]]),
                _ => self.reshape(vec![self.get_shape()[0], 1]),
            }
        }
    }

    fn atleast_3d(&self) -> Result<Self, ArrayError> {
        if self.ndim() >= 3 { Ok(self.clone()) }
        else {
            match self.ndim() {
                0 => self.reshape(vec![1, 1, 1]),
                1 => self.reshape(vec![1, self.get_shape()[0], 1]),
                2 => self.reshape(vec![self.get_shape()[0], self.get_shape()[1], 1]),
                _ => Ok(self.clone()),
            }
        }
    }
}
