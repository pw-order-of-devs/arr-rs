use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Sum, Product, Diff functions
pub trait ArraySumProdDiff<N: NumericOps> where Self: Sized + Clone {

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![1, 2, 6, 24]), arr.cumprod(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]), arr.cumsum(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::flat(vec![1., 2., 6., 24., 24.]), arr.nancumprod(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]);
    /// assert_eq!(Array::flat(vec![1., 3., 6., 10., 10.]), arr.nancumsum(None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn nancumsum(&self, axis: Option<isize>) -> Result<Array<N>, ArrayError>;

    /// The differences between consecutive elements of an array
    ///
    /// # Arguments
    ///
    /// * `n` - number of times values are differenced
    /// * `axis` - the axis along which to execute the function. optional. defaults to last axis
    /// * `append` - number(s) to append at the end along axis prior to performing the difference
    /// * `prepend` - number(s) to append at the beginning along axis prior to performing the difference
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 4., 4., 7.]);
    /// assert_eq!(Array::flat(vec![1., 2., 0., 3.]), arr.diff(1, None, None, None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn diff(&self, n: usize, axis: Option<isize>, prepend: Option<Array<N>>, append: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

    /// The differences between consecutive elements of an array
    ///
    /// # Arguments
    ///
    /// * `to_end` - number(s) to append at the end of the returned differences
    /// * `to_begin` - number(s) to append at the beginning of the returned differences
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 4., 4., 7.]);
    /// assert_eq!(Array::flat(vec![1., 2., 0., 3.]), arr.ediff1d(None, None));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn ediff1d(&self, to_end: Option<Array<N>>, to_begin: Option<Array<N>>) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArraySumProdDiff<N> for Array<N> {

    fn prod(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.prod(None));
            result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
        } else {
            Self::single(self.elements.iter().fold(N::one(), |acc, &x| acc * x))
        }
    }

    fn sum(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.sum(None));
            result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
        } else {
            Self::single(self.elements.iter().fold(N::zero(), |acc, &x| acc + x))
        }
    }

    fn nanprod(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.nanprod(None));
                result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
        } else {
            Self::single(self.elements.iter().fold(N::one(), |acc, &x|
                acc * if x.to_f64().is_nan() { N::one() } else { x }
            ))
        }
    }

    fn nansum(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            let result = self.apply_along_axis(axis, |arr| arr.nansum(None));
                result.reshape(&result.get_shape()?.remove_at_if(axis, result.ndim()? > 1))
        } else {
            Self::single(self.elements.iter().fold(N::zero(), |acc, &x|
                acc + if x.to_f64().is_nan() { N::zero() } else { x }
            ))
        }
    }

    fn cumprod(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.cumprod(None))
        } else {
            let mut acc = N::one();
            self.ravel()?.map(|&x| {
                acc *= x;
                acc
            })
        }
    }

    fn cumsum(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.cumsum(None))
        } else {
            let mut acc = N::zero();
            self.ravel()?.map(|&x| {
                acc += x;
                acc
            })
        }
    }

    fn nancumprod(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.nancumprod(None))
        } else {
            let mut acc = N::one();
            self.ravel()?.map(|&x| {
                acc *= if x.to_f64().is_nan() { N::one() } else { x };
                acc
            })
        }
    }

    fn nancumsum(&self, axis: Option<isize>) -> Result<Self, ArrayError> {
        if let Some(axis) = axis {
            let axis = self.normalize_axis(axis);
            self.apply_along_axis(axis, |arr| arr.nancumsum(None))
        } else {
            let mut acc = N::zero();
            self.ravel()?.map(|&x| {
                acc += if x.to_f64().is_nan() { N::zero() } else { x };
                acc
            })
        }
    }

    fn diff(&self, n: usize, axis: Option<isize>, prepend: Option<Self>, append: Option<Self>) -> Result<Self, ArrayError> {
        if n == 0 {
            Self::empty()
        } else if self.ndim()? == 1 {
            let mut elements = prepend.unwrap_or(Self::empty()?).get_elements()?;
            elements.extend_from_slice(&self.get_elements()?);
            elements.extend_from_slice(&append.unwrap_or(Self::empty()?).get_elements()?);
            for _ in 0..n { elements = Self::flat(elements.clone()).ediff1d(None, None).get_elements()? }
            Self::flat(elements)
        } else {
            fn diff_extend_partial<N: Numeric>(array: &Array<N>, partial: Vec<Array<N>>, other: Option<Array<N>>, axis: usize, rev: bool) -> Result<Vec<Array<N>>, ArrayError> {
                if other.is_none() {return Ok(partial) }
                let other = other.unwrap();
                array.ndim()?.is_equal(&other.ndim()?)?;
                array.get_shape()?.remove_at(axis).is_equal(&other.get_shape()?.remove_at(axis))?;
                let p_partial = other
                    .moveaxis(vec![axis.to_isize()], vec![array.ndim()?.to_isize()])
                    .ravel().split(other.get_shape()?.remove_at(axis).into_iter().product(), None)?;
                let mut tmp_v = vec![partial, p_partial];
                if rev { tmp_v.reverse() };
                let result = tmp_v[0].clone().into_iter().zip(&tmp_v[1]).map(|(arr, other)| {
                    let mut elements = other.elements.clone();
                    elements.extend_from_slice(&arr.elements);
                    Array::flat(elements).unwrap()
                }).collect::<Vec<Array<N>>>();
                Ok(result)
            }

            let axis = axis.unwrap_or(-1);
            let axis = self.normalize_axis(axis);

            let parts = self.get_shape()?.remove_at(axis).into_iter().product();
            let mut partial = self
                .moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()])?
                .ravel().split(parts, None)?;

            partial = diff_extend_partial(self, partial, prepend.clone(), axis, false)?;
            partial = diff_extend_partial(self, partial, append.clone(), axis, true)?;

            let mut new_shape = self.get_shape()?;
            if let Some(p) = prepend { new_shape[axis] += p.get_shape()?[axis] };
            if let Some(a) = append { new_shape[axis] += a.get_shape()?[axis] };

            let array = partial.into_iter()
                .flatten()
                .collect::<Self<>>()
                .reshape(&new_shape.swap_ext(axis, self.ndim()? - 1));
            let array =
                if axis == 0 { array.transpose(None) }
                else { array.moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()]) };

            array.apply_along_axis(axis, |arr| arr.diff(n, None, None, None))
        }
    }

    fn ediff1d(&self, to_end: Option<Self>, to_begin: Option<Self>) -> Result<Self, ArrayError> {
        let array = self.ravel()?;
        let (to_end, to_begin) = (to_end.unwrap_or(Self::empty()?), to_begin.unwrap_or(Self::empty()?));
        let diffs = (1..array.len()?).map(|i| array[i] - array[i - 1]).collect::<Vec<N>>();
        let mut result = to_begin.get_elements()?;
        result.extend_from_slice(&diffs);
        result.extend_from_slice(&to_end.get_elements()?);
        Self::flat(result)
    }
}

impl <N: NumericOps> ArraySumProdDiff<N> for Result<Array<N>, ArrayError> {

    fn prod(&self, axis: Option<isize>) -> Self {
        self.clone()?.prod(axis)
    }

    fn sum(&self, axis: Option<isize>) -> Self {
        self.clone()?.sum(axis)
    }

    fn nanprod(&self, axis: Option<isize>) -> Self {
        self.clone()?.nanprod(axis)
    }

    fn nansum(&self, axis: Option<isize>) -> Self {
        self.clone()?.nansum(axis)
    }

    fn cumprod(&self, axis: Option<isize>) -> Self {
        self.clone()?.cumprod(axis)
    }

    fn cumsum(&self, axis: Option<isize>) -> Self {
        self.clone()?.cumsum(axis)
    }

    fn nancumprod(&self, axis: Option<isize>) -> Self {
        self.clone()?.nancumprod(axis)
    }

    fn nancumsum(&self, axis: Option<isize>) -> Self {
        self.clone()?.nancumsum(axis)
    }

    fn diff(&self, n: usize, axis: Option<isize>, prepend: Option<Array<N>>, append: Option<Array<N>>) -> Self {
        self.clone()?.diff(n, axis, prepend, append)
    }

    fn ediff1d(&self, to_end: Option<Array<N>>, to_begin: Option<Array<N>>) -> Self {
        self.clone()?.ediff1d(to_end, to_begin)
    }
}
