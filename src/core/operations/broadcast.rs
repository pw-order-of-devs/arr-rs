use crate::{
    core::prelude::*,
    errors::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Broadcast functions
pub trait ArrayBroadcast<T: ArrayElement> where Self: Sized + Clone {

    /// Broadcast an array to a new shape
    ///
    /// # Arguments
    ///
    /// * `other` - other array for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<Tuple2<i32>> = Array::new(vec![
    ///     (1, 4), (1, 5), (1, 6),
    ///     (2, 4), (2, 5), (2, 6),
    ///     (3, 4), (3, 5), (3, 6)
    /// ].into_iter().map(Tuple2::from_tuple).collect(), vec![3, 3]).unwrap();
    ///
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let arr_2: Array<i32> = array!([[4, 5, 6]]).unwrap();
    ///
    /// let broadcast: Array<Tuple2<i32>> = arr_1.broadcast(&arr_2).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast(&self, other: &Array<T>) -> Result<Array<Tuple2<T>>, ArrayError>;

    /// Broadcast an array to a new shape
    ///
    /// # Arguments
    ///
    /// * `other` - other array for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = Array::new(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], vec![3, 3]).unwrap();
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    ///
    /// let broadcast: Array<i32> = arr_1.broadcast_to(vec![3, 3]).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast_to(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError>;

    /// Broadcast a list of arrays to a common shape
    ///
    /// # Arguments
    ///
    /// * `arrays` - list of arrays for broadcasting
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Vec<Array<i32>> = vec![
    ///     Array::new(vec![1, 1, 1, 2, 2, 2, 3, 3, 3], vec![3, 3]).unwrap(),
    ///     Array::new(vec![4, 5, 6, 4, 5, 6, 4, 5, 6], vec![3, 3]).unwrap(),
    /// ];
    /// let arr_1: Array<i32> = array!([[1], [2], [3]]).unwrap();
    /// let arr_2: Array<i32> = array!([4, 5, 6]).unwrap();
    ///
    /// let broadcast: Vec<Array<i32>> = Array::broadcast_arrays(vec![arr_1 ,arr_2]).unwrap();
    /// assert_eq!(expected, broadcast);
    /// ```
    fn broadcast_arrays(arrays: Vec<Array<T>>) -> Result<Vec<Array<T>>, ArrayError>;
}

impl <T: ArrayElement> ArrayBroadcast<T> for Array<T> {

    fn broadcast(&self, other: &Array<T>) -> Result<Array<Tuple2<T>>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;

        let final_shape = self.broadcast_shape(other.get_shape()?)?;

        let inner_arrays_self = self.extract_inner_arrays();
        let inner_arrays_other = other.extract_inner_arrays();

        let output_elements = inner_arrays_self.iter().cycle()
            .zip(inner_arrays_other.iter().cycle())
            .flat_map( | (inner_self, inner_other) | match (inner_self.len(), inner_other.len()) {
                (1, _) => inner_self.iter().cycle()
                    .zip(inner_other.iter())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(a, b) | Tuple2(a.clone(), b.clone()))
                    .collect::< Vec < _ > > (),
                (_, 1) => inner_self.iter()
                    .zip(inner_other.iter().cycle())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(a, b) | Tuple2(a.clone(), b.clone()))
                    .collect::<Vec < _ > > (),
                _ => inner_self.iter().cycle()
                    .zip(inner_other.iter().cycle())
                    .take(final_shape[final_shape.len() - 1])
                    .map(|(a, b) | Tuple2(a.clone(), b.clone()))
                    .collect::< Vec< _ > > (),
            })
            .take(final_shape.iter().product())
            .collect:: < Vec<_ > > ();

        Array::new(output_elements, final_shape)
    }

    fn broadcast_to(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        self.get_shape()?.is_broadcastable(&shape)?;

        if self.get_shape()?.iter().product::<usize>() == shape.iter().product::<usize>() {
            self.reshape(shape)
        } else {
            let output_elements: Vec<T> = self.elements
                .chunks_exact(self.shape[self.shape.len() - 1])
                .flat_map(|inner| {
                    let extended_inner = inner.iter()
                        .cycle()
                        .take(shape[shape.len() - 1])
                        .cloned()
                        .collect::<Vec<T>>();
                    extended_inner.into_iter()
                })
                .cycle()
                .take(shape.iter().product())
                .collect();

            Array::new(output_elements, shape)
        }
    }

    fn broadcast_arrays(arrays: Vec<Array<T>>) -> Result<Vec<Array<T>>, ArrayError> {
        arrays.iter().map(|array| array.get_shape()).collect::<Vec<Result<Vec<usize>, ArrayError>>>().has_error()?;
        let shapes = arrays.iter()
            .map(|array| array.get_shape().unwrap())
            .collect::<Vec<_>>();

        let common_shape = Self::common_broadcast_shape(&shapes);
        if let Ok(common_shape) = common_shape {
            let result = arrays.iter()
                .map(|array| array.broadcast_to(common_shape.clone()))
                .collect::<Vec<Result<Self, _>>>()
                .has_error()?
                .into_iter().map(|a| a.unwrap())
                .collect();
            Ok(result)
        } else {
            Err(common_shape.err().unwrap())
        }
    }
}

impl <T: ArrayElement> ArrayBroadcast<T> for Result<Array<T>, ArrayError> {

    fn broadcast(&self, other: &Array<T>) -> Result<Array<Tuple2<T>>, ArrayError> {
        self.clone()?.broadcast(other)
    }

    fn broadcast_to(&self, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.broadcast_to(shape)
    }

    fn broadcast_arrays(arrays: Vec<Array<T>>) -> Result<Vec<Array<T>>, ArrayError> {
        Array::broadcast_arrays(arrays)
    }
}

impl <T: ArrayElement> Array<T> {

    fn broadcast_shape(&self, shape: Vec<usize>) -> Result<Vec<usize>, ArrayError> {
        let max_dim = self.shape.len().max(shape.len());
        let shape1_padded = self.shape.iter().rev()
            .copied().chain(std::iter::repeat(1))
            .take(max_dim);
        let shape2_padded = shape.iter().rev()
            .copied().chain(std::iter::repeat(1))
            .take(max_dim);

        let zipped = shape1_padded.zip(shape2_padded.into_iter());
        let result = zipped
            .map(|(dim1, dim2)| {
                if dim1 == 1 { Ok(dim2) }
                else if dim2 == 1 || dim1 == dim2 { Ok(dim1) }
                else { Err(ArrayError::BroadcastShapeMismatch) }
            })
            .collect::<Vec<Result<usize, ArrayError>>>()
            .has_error()?.iter()
            .map(|a| *a.as_ref().unwrap())
            .collect();
        Ok(result)
    }

    fn common_broadcast_shape(shapes: &[Vec<usize>]) -> Result<Vec<usize>, ArrayError> {
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

        if is_compatible { Ok(common_shape.into_iter().rev().collect()) }
        else { Err(ArrayError::BroadcastShapeMismatch) }
    }

    fn extract_inner_arrays(&self) -> Vec<Vec<T>> {
        match self.shape.len() {
            1 => vec![self.elements.clone()],
            _ => self.elements
                .chunks_exact(*self.shape.last().unwrap())
                .map(Vec::from)
                .collect(),
        }
    }

    pub(crate) fn broadcast_h2<S: ArrayElement>(&self, other: &Array<S>) -> Result<TupleH2<T, S>, ArrayError> {
        let tmp_other = Array::single(T::zero()).broadcast_to(other.get_shape()?)?;
        let tmp_array = self.broadcast(&tmp_other)?;

        let array = tmp_array.clone().into_iter()
            .map(|t| t.0).collect::<Array<T>>()
            .reshape(tmp_array.get_shape()?)?;
        let other = other.broadcast_to(array.get_shape()?)?;

        Ok((array, other))
    }

    pub(crate) fn broadcast_h3<S: ArrayElement, Q: ArrayElement>(&self, other_1: &Array<S>, other_2: &Array<Q>) -> Result<TupleH3<T, S, Q>, ArrayError> {
        let tmp_other_1 = Array::single(T::zero()).broadcast_to(other_1.get_shape()?)?;
        let tmp_other_2 = Array::single(T::zero()).broadcast_to(other_2.get_shape()?)?;
        let broadcasted = Self::broadcast_arrays(vec![self.clone(), tmp_other_1, tmp_other_2])?;

        let array = broadcasted[0].clone();
        let other_1 = other_1.broadcast_to(array.get_shape()?)?;
        let other_2 = other_2.broadcast_to(array.get_shape()?)?;

        Ok((array, other_1, other_2))
    }
}
