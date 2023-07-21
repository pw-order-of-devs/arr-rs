use crate::{
    core::prelude::*,
    errors::prelude::*,
};

/// ArrayTrait - Array Tiling functions
pub trait ArrayTiling<T: ArrayElement> where Self: Sized + Clone {

    /// Repeat each element of an array after themselves
    ///
    /// # Arguments
    ///
    /// * `repeats` - number of repetitions for each element, broadcasted to fit the shape of the given axis
    /// * `axis` - the axis along which to repeat. optional, if None, array is flattened
    ///
    /// # Examples
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = array!(3);
    /// assert_eq!(array![3, 3, 3, 3], arr.repeat(&array!(4).unwrap(), None));
    ///
    /// let arr = array!([[1, 2], [3, 4]]);
    /// assert_eq!(array!([[1, 2], [3, 4], [3, 4]]), arr.repeat(&array!([1, 2]).unwrap(), Some(0)));
    /// ```
    fn repeat(&self, repeats: &Array<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayTiling<T> for Array<T> {

    fn repeat(&self, repeats: &Array<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        if let Some(axis) = axis {
            let mut new_shape = self.get_shape()?;
            new_shape[axis] = repeats.clone().into_iter().sum();
            let mut tmp_shape = new_shape.clone();
            tmp_shape.swap(0, axis);
            self.split(self.get_shape()?[axis], Some(axis))?.into_iter()
                .zip(&repeats.broadcast_to(vec![self.get_shape()?[axis]]).get_elements()?)
                .flat_map(|(el, &rep)| vec![el; rep])
                .collect::<Vec<Array<T>>>()
                .into_iter().flatten()
                .collect::<Array<T>>()
                .reshape(tmp_shape.clone())
                .moveaxis(vec![0], vec![axis as isize])
                .reshape(new_shape)
        } else {
            let result = self.get_elements()?.into_iter()
                .zip(&repeats.broadcast_to(self.get_shape()?).get_elements()?)
                .flat_map(|(el, &rep)| vec![el; rep])
                .collect();
            Array::flat(result)
        }
    }
}

impl <T: ArrayElement> ArrayTiling<T> for Result<Array<T>, ArrayError> {

    fn repeat(&self, repeats: &Array<usize>, axis: Option<usize>) -> Result<Array<T>, ArrayError> {
        self.clone()?.repeat(repeats, axis)
    }
}
