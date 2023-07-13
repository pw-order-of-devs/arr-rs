use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::vec_ext::VecRemoveAt,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Folding functions
pub trait ArrayFolding<N: Numeric> where Self: Sized + Clone {

    /// Multiplication of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::single(24), arr.prod(None));
    /// ```
    fn prod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Sum of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::single(10), arr.sum(None));
    /// ```
    fn sum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Sum of array elements treating NaN as one
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::single(24.), arr.nanprod(None));
    /// ```
    fn nanprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Sum of array elements treating NaN as zero
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::single(10.), arr.nansum(None));
    /// ```
    fn nansum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Cumulative product of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 2, 6, 24]), arr.cumprod(None));
    /// ```
    fn cumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]), arr.cumsum(None));
    /// ```
    fn cumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Cumulative product of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(Array::flat(vec![1., 2., 6., 24., 24.]), arr.nancumprod(None));
    /// ```
    fn nancumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Arguments
    ///
    /// * `axis` - the axis along which to execute the function. optional. if negative, counts from last to first axis. if None, array is raveled
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(Array::flat(vec![1., 3., 6., 10., 10.]), arr.nancumsum(None));
    /// ```
    fn nancumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayFolding<N> for Array<N> {

    fn prod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal(axis, |arr| arr.prod(None))
        } else {
            Array::single(self.elements.iter().fold(N::one(), |acc, x| N::from(acc.to_f64() * x.to_f64())))
        }
    }

    fn sum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal(axis, |arr| arr.sum(None))
        } else {
            Array::single(self.elements.iter().fold(N::zero(), |acc, x| N::from(acc.to_f64() + x.to_f64())))
        }
    }

    fn nanprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal(axis, |arr| arr.nanprod(None))
        } else {
            Array::single(self.elements.iter().fold(N::one(), |acc, x| {
                let x = if x.to_f64().is_nan() { 1. } else { x.to_f64() };
                N::from(acc.to_f64() * x)
            }))
        }
    }

    fn nansum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal(axis, |arr| arr.nansum(None))
        } else {
            Array::single(self.elements.iter().fold(N::zero(), |acc, x| {
                let x = if x.to_f64().is_nan() { 0. } else { x.to_f64() };
                N::from(acc.to_f64() + x)
            }))
        }
    }

    fn cumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal_cum(axis, |arr| arr.cumprod(None))
        } else {
            let mut acc = N::one();
            self.ravel()?.map(|&x| {
                acc = N::from(acc.to_f64() * x.to_f64());
                acc
            })
        }
    }

    fn cumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal_cum(axis, |arr| arr.cumsum(None))
        } else {
            let mut acc = N::zero();
            self.ravel()?.map(|&x| {
                acc = N::from(acc.to_f64() + x.to_f64());
                acc
            })
        }
    }

    fn nancumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal_cum(axis, |arr| arr.nancumprod(None))
        } else {
            let mut acc = N::one();
            self.ravel()?.map(|&x| {
                let x = if x.to_f64().is_nan() { 1. } else { x.to_f64() };
                acc = N::from(acc.to_f64() * x);
                acc
            })
        }
    }

    fn nancumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        if let Some(axis) = axis {
            self.folding_internal_cum(axis, |arr| arr.nancumsum(None))
        } else {
            let mut acc = N::zero();
            self.ravel()?.map(|&x| {
                let x = if x.to_f64().is_nan() { 0. } else { x.to_f64() };
                acc = N::from(acc.to_f64() + x);
                acc
            })
        }
    }
}

impl <N: Numeric> ArrayFolding<N> for Result<Array<N>, ArrayError> {

    fn prod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.prod(axis)
    }

    fn sum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.sum(axis)
    }

    fn nanprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.nanprod(axis)
    }

    fn nansum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.nansum(axis)
    }

    fn cumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumprod(axis)
    }

    fn cumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumsum(axis)
    }

    fn nancumprod(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.nancumprod(axis)
    }

    fn nancumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.nancumsum(axis)
    }
}

impl <N: Numeric> Array<N> {

    fn folding_internal<F: FnMut(&Array<N>) -> Result<Array<N>, ArrayError>>(&self, axis: isize, f: F) -> Result<Array<N>, ArrayError> {
        let axis = Self::normalize_axis(axis, self.ndim()?);
        let new_shape = self.get_shape()?.remove_at(axis);
        let parts = self.get_shape()?.remove_at(axis).into_iter().product();
        let partial = self.moveaxis(vec![axis as isize], vec![self.ndim()? as isize])?;
        Array::folding_partial(&partial, parts, f)
            .reshape(new_shape.clone())
    }

    fn folding_internal_cum<F>(&self, axis: isize, f: F) -> Result<Array<N>, ArrayError>
        where F: FnMut(&Array<N>) -> Result<Array<N>, ArrayError> {
        let axis = Self::normalize_axis(axis, self.ndim()?);
        let parts = self.get_shape()?.remove_at(axis).into_iter().product();
        let partial = self.moveaxis(vec![axis as isize], vec![self.ndim()? as isize])?;
        let partial = Array::folding_partial(&partial, parts, f)
            .reshape(partial.get_shape()?);
        if axis == 0 { partial.rollaxis((self.ndim()? - 1) as isize, None) }
        else { partial.moveaxis(vec![axis as isize], vec![self.ndim()? as isize]) }
            .reshape(self.get_shape()?)
    }

    fn folding_partial<F>(partial: &Array<N>, parts: usize, mut f: F) -> Result<Array<N>, ArrayError>
        where F: FnMut(&Array<N>) -> Result<Array<N>, ArrayError> {
        let result = partial
            .ravel()
            .split(parts, None)?.into_iter()
            .map(|arr| f(&arr))
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?.into_iter()
            .flat_map(|arr| arr.unwrap())
            .collect::<Array<N>>();
        Ok(result)
    }
}
