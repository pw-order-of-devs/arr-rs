use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};

/// `ArrayTrait` - Array Indexing functions
pub trait ArrayIndexing<T: ArrayElement> where Self: Sized + Clone {

    /// Return an index of element at the given coordinates
    ///
    /// # Arguments
    ///
    /// * `coords` - vector representing the coordinates of the element in array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let idx_1 = arr.index_at(&[0, 0, 0]).unwrap();
    /// assert_eq!(0, idx_1);
    ///
    /// let idx_2 = arr.index_at(&[1, 0, 1]).unwrap();
    /// assert_eq!(5, idx_2);
    ///
    /// let idx_3 = arr.index_at(&[1, 1, 1]).unwrap();
    /// assert_eq!(7, idx_3);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn index_at(&self, coords: &[usize]) -> Result<usize, ArrayError>;

    /// Return coordinates at the given index of element
    ///
    /// # Arguments
    ///
    /// * `index` - index of element in flattened array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let coord_1 = arr.index_to_coord(0).unwrap();
    /// assert_eq!(vec![0, 0, 0], coord_1);
    ///
    /// let coord_2 = arr.index_to_coord(5).unwrap();
    /// assert_eq!(vec![1, 0, 1], coord_2);
    ///
    /// let coord_3 = arr.index_to_coord(7).unwrap();
    /// assert_eq!(vec![1, 1, 1], coord_3);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn index_to_coord(&self, idx: usize) -> Result<Vec<usize>, ArrayError>;

    /// Return an index of element at the given coordinates
    ///
    /// # Arguments
    ///
    /// * `coords` - vector representing the coordinates of the element in array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<i32> = Array::new(vec![1,2,3,4,5,6,7,8], vec![2, 2, 2]).unwrap();
    ///
    /// let at_1 = arr.at(&[0, 0, 0]).unwrap();
    /// assert_eq!(1, at_1);
    ///
    /// let at_2 = arr.at(&[1, 0, 1]).unwrap();
    /// assert_eq!(6, at_2);
    ///
    /// let at_3 = arr.at(&[1, 1, 1]).unwrap();
    /// assert_eq!(8, at_3);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn at(&self, coords: &[usize]) -> Result<T, ArrayError>;

    /// Return a subarray of provided range
    ///
    /// # Arguments
    ///
    /// * `range` - starting and ending indices of elements to include in the subarray
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::flat(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    /// let expected = Array::<i32>::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(expected, arr.slice(0..4).unwrap());
    ///
    /// let arr = Array::<i32>::new(vec![1, 2, 3, 4, 5, 6, 7, 8], vec![2, 4]).unwrap();
    /// let expected = Array::<i32>::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(expected, arr.slice(0..1).unwrap());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn slice(&self, range: std::ops::Range<usize>) -> Result<Array<T>, ArrayError>;

    /// Return a subarray consisting on values on given indices.
    ///
    /// # Arguments
    ///
    /// * `indices` - indices which should be included in resulting array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::<i32>::flat(vec![1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
    ///
    /// let expected = Array::<i32>::flat(vec![3, 5, 7]).unwrap();
    /// let slice_1 = arr.indices_at(&[2, 4, 6]).unwrap();
    /// assert_eq!(format!("{expected}"), format!("{slice_1}"));
    ///
    /// let expected = Array::<i32>::flat(vec![4, 5, 3, 8, 6, 7, 1, 2]).unwrap();
    /// let slice_1 = arr.indices_at(&[3, 4, 2, 7, 5, 6, 0, 1]).unwrap();
    /// assert_eq!(format!("{expected}"), format!("{slice_1}"));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn indices_at(&self, indices: &[usize]) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayIndexing<T> for Array<T> {

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
        if idx >= self.len()? {
            Err(ArrayError::ParameterError { param: "idx", message: "index must be smaller than array length", })
        } else {
            let result = self.shape.iter().rev().fold((idx, Vec::new()), |(ri, mut coords), &dim| {
                coords.push(ri % dim);
                (ri / dim, coords)
            }).1.into_iter().rev().collect();
            Ok(result)
        }
    }

    fn at(&self, coords: &[usize]) -> Result<T, ArrayError> {
        match self.index_at(coords) {
            Ok(idx) => Ok(self.elements[idx].clone()),
            Err(e) => Err(e),
        }
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Result<Self, ArrayError> {
        if !(range.start <= range.end && range.end <= self.elements.len()) {
            return Err(ArrayError::OutOfBounds { value: "slice range" })
        }

        if self.shape.len() == 1 {
            Self::flat(self.elements[range].into())
        } else if range.len() >= self.shape[0] {
            Ok(self.clone())
        } else {
            let new_shape =
                if range.len() > 1 { vec![range.len()].into_iter().chain(self.shape[1..].iter().copied()).collect() }
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

    fn indices_at(&self, indices: &[usize]) -> Result<Self, ArrayError> {
        if self.ndim()? == 1 {
            for &i in indices {
                if i >= self.len()? { return Err(ArrayError::OutOfBounds { value: "indices" }) }
            }
            indices.iter()
                .map(|&i| self[i].clone())
                .collect::<Vec<T>>()
                .to_array()
        } else {
            let arrs = self.split_axis(0)?;
            for &i in indices {
                if i >= arrs.len() { return Err(ArrayError::OutOfBounds { value: "indices" }) }
            }
            let new_shape = self.get_shape()?.update_at(0, indices.len());
            indices.iter()
                .flat_map(|&i| arrs[i].clone())
                .collect::<Vec<T>>()
                .to_array()
                .reshape(&new_shape)
        }
    }
}

impl <T: ArrayElement> ArrayIndexing<T> for Result<Array<T>, ArrayError> {

    fn index_at(&self, coords: &[usize]) -> Result<usize, ArrayError> {
        self.clone()?.index_at(coords)
    }

    fn index_to_coord(&self, idx: usize) -> Result<Vec<usize>, ArrayError> {
        self.clone()?.index_to_coord(idx)
    }

    fn at(&self, coords: &[usize]) -> Result<T, ArrayError> {
        self.clone()?.at(coords)
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        self.clone()?.slice(range)
    }

    fn indices_at(&self, indices: &[usize]) -> Self {
        self.clone()?.indices_at(indices)
    }
}
