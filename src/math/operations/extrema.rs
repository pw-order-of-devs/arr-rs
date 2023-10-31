use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    numeric::prelude::*,
};

/// `ArrayTrait` - Array Extrema functions
pub trait ArrayExtrema<N: Numeric> where Self: Sized + Clone {

    /// Element-wise maximum of array elements
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(
    /// format!("{:#}", Array::flat(vec![2., f64::NAN, 3., 10.]).unwrap()),
    /// format!("{:#}", arr.maximum(&Array::flat(vec![2., f64::NAN, 2., 10.]).unwrap()).unwrap())
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn maximum(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the maximum of an array or maximum along an axis
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::single(4.), arr.max(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn max(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return the maximum of an array or maximum along an axis
    /// alias on `max`
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::single(4.), arr.amax(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn amax(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Element-wise maximum of array elements
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(
    /// format!("{:#}", Array::flat(vec![2., 2., 3., 10.]).unwrap()),
    /// format!("{:#}", arr.fmax(&Array::flat(vec![2., f64::NAN, 2., 10.]).unwrap()).unwrap())
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn fmax(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the maximum of an array or maximum along an axis, ignoring NAN
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    //
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::single(4.), arr.nanmax(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn nanmax(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Element-wise minimum of array elements
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(
    /// format!("{:#}", Array::flat(vec![1., f64::NAN, 2., 4.]).unwrap()),
    /// format!("{:#}", arr.minimum(&Array::flat(vec![2., f64::NAN, 2., 10.]).unwrap()).unwrap())
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn minimum(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the minimum of an array or minimum along an axis
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::single(1.), arr.min(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn min(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Return the minimum of an array or minimum along an axis
    /// alias on `min`
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::single(1.), arr.amin(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn amin(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Element-wise maximum of array elements
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(
    /// format!("{:#}", Array::flat(vec![1., 2., 2., 4.]).unwrap()),
    /// format!("{:#}", arr.fmin(&Array::flat(vec![2., f64::NAN, 2., 10.]).unwrap()).unwrap())
    /// );
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn fmin(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the minimum of an array or minimum along an axis, ignoring NAN
    ///
    /// # Arguments
    ///
    /// * `axis` - axis along which to operate. optional, if None, input is flattened
    //
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::single(1.), arr.nanmin(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn nanmin(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayExtrema<N> for Array<N> {

    fn maximum(&self, other: &Self) -> Result<Self, ArrayError> {
        self.zip(other)?
            .map(|item| {
                if item.0.to_f64().is_nan() || item.1.to_f64().is_nan() { N::from(f64::NAN) }
                else { N::from(f64::max(item.0.to_f64(), item.1.to_f64())) }
            })
    }

    fn max(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        match axis {
            Some(axis) => {
                let axis = self.normalize_axis(axis);
                let result = self.apply_along_axis(axis, |arr| arr.max(None));
                result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
            },
            None => {
                if self.to_array_f64().get_elements()?.iter().any(ArrayElement::is_nan) {
                    Self::single(N::from(f64::NAN))
                } else {
                    let result = self.into_iter().fold(self[0], |a, &b| if a < b { b } else { a });
                    Self::single(result)
                }
            }
        }
    }

    fn amax(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        self.max(axis)
    }

    fn fmax(&self, other: &Self) -> Result<Self, ArrayError> {
        self.zip(other)?
            .map(|item| N::from(f64::max(item.0.to_f64(), item.1.to_f64())))
    }

    fn nanmax(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        match axis {
            Some(axis) => {
                let axis = self.normalize_axis(axis);
                let result = self.apply_along_axis(axis, |arr| arr.nanmax(None));
                result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
            },
            None => {
                if self.to_array_f64().get_elements()?.iter().all(ArrayElement::is_nan) {
                    Self::single(N::from(f64::NAN))
                } else {
                    let filtered = self.get_elements()?.into_iter().filter(|i| !i.to_f64().is_nan()).collect::<Self>();
                    let result = filtered.fold(filtered[0], |&a, &b| if a < b { b } else { a })?;
                    Self::single(result)
                }
            }
        }
    }

    fn minimum(&self, other: &Self) -> Result<Self, ArrayError> {
        self.zip(other)?
            .map(|item| {
                if item.0.to_f64().is_nan() || item.1.to_f64().is_nan() { N::from(f64::NAN) }
                else { N::from(f64::min(item.0.to_f64(), item.1.to_f64())) }
            })
    }

    fn min(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.min(None));
            result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
        } else {
            if self.to_array_f64().get_elements()?.iter().any(ArrayElement::is_nan) { return Self::single(N::from(f64::NAN)) }
            let result = self.into_iter().fold(self[0], |a, &b| if a > b { b } else { a });
            Self::single(result)
        }
    }

    fn amin(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        self.min(axis)
    }

    fn fmin(&self, other: &Self) -> Result<Self, ArrayError> {
        self.zip(other)?
            .map(|item| N::from(f64::min(item.0.to_f64(), item.1.to_f64())))
    }

    fn nanmin(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        match axis {
            Some(axis) => {
                let axis = self.normalize_axis(axis);
                let result = self.apply_along_axis(axis, |arr| arr.nanmin(None));
                result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
            },
            None => {
                if self.to_array_f64().get_elements()?.iter().all(ArrayElement::is_nan) {
                    Self::single(N::from(f64::NAN))
                } else {
                    let filtered = self.get_elements()?.into_iter().filter(|i| !i.to_f64().is_nan()).collect::<Self>();
                    let result = filtered.fold(filtered[0], |&a, &b| if a > b { b } else { a })?;
                    Self::single(result)
                }
            }
        }
    }
}

impl <N: Numeric> ArrayExtrema<N> for Result<Array<N>, ArrayError> {

    fn maximum(&self, other: &Array<N>) -> Self {
        self.clone()?.maximum(other)
    }

    fn max(&self, axis: Option<isize>) -> Self {
        self.clone()?.max(axis)
    }

    fn amax(&self, axis: Option<isize>) -> Self {
        self.clone()?.amax(axis)
    }

    fn fmax(&self, other: &Array<N>) -> Self {
        self.clone()?.fmax(other)
    }

    fn nanmax(&self, axis: Option<isize>) -> Self {
        self.clone()?.nanmax(axis)
    }

    fn minimum(&self, other: &Array<N>) -> Self {
        self.clone()?.minimum(other)
    }

    fn min(&self, axis: Option<isize>) -> Self {
        self.clone()?.min(axis)
    }

    fn amin(&self, axis: Option<isize>) -> Self {
        self.clone()?.amin(axis)
    }

    fn fmin(&self, other: &Array<N>) -> Self {
        self.clone()?.fmin(other)
    }

    fn nanmin(&self, axis: Option<isize>) -> Self {
        self.clone()?.nanmin(axis)
    }
}
