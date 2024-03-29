use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
};
use crate::prelude::Numeric;

/// `ArrayTrait` - Array Tiling functions
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
    /// let arr = Array::<i32>::single(3);
    /// assert_eq!(array![i32, 3, 3, 3, 3], arr.repeat(&vec![4], None));
    ///
    /// let arr = Array::<i32>::new(vec![1, 2, 3, 4], vec![2, 2]);
    /// assert_eq!(array!(i32, [[1, 2], [3, 4], [3, 4]]), arr.repeat(&vec![1, 2], Some(0)));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn repeat(&self, repeats: &[usize], axis: Option<usize>) -> Result<Array<T>, ArrayError>;
}

impl <T: ArrayElement> ArrayTiling<T> for Array<T> {

    fn repeat(&self, repeats: &[usize], axis: Option<usize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let repeats = repeats.to_vec().to_array()?.broadcast_to(vec![self.get_shape()?[axis]]).get_elements()?;
            let new_axis_len = repeats.clone().into_iter().sum();
            let new_shape = self.get_shape()?.update_at(axis, new_axis_len);
            let tmp_shape = new_shape.clone().swap_ext(0, axis);
            let partial = self.split(self.get_shape()?[axis], Some(axis))?.into_iter()
                .zip(&repeats)
                .flat_map(|(el, &rep)| vec![el; rep])
                .flatten()
                .collect::<Self>();
            partial.reshape(&tmp_shape)
                .moveaxis(vec![0], vec![axis.to_isize()])
                .reshape(&new_shape)
        } else {
            let result = self.get_elements()?.into_iter()
                .zip(&repeats.to_vec().to_array()?.broadcast_to(self.get_shape()?).get_elements()?)
                .flat_map(|(el, &rep)| vec![el; rep])
                .collect();
            Self::flat(result)
        }
    }
}

impl <T: ArrayElement> ArrayTiling<T> for Result<Array<T>, ArrayError> {

    fn repeat(&self, repeats: &[usize], axis: Option<usize>) -> Self {
        self.clone()?.repeat(repeats, axis)
    }
}
