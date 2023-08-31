use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Math Misc functions
pub trait ArrayMathMisc<N: Numeric> where Self: Sized + Clone {

    /// Returns the discrete, linear convolution of two one-dimensional sequences
    /// arrays are flattened for computation
    ///
    /// # Arguments
    ///
    /// * `other` - array to perform the operation with
    /// * `mode` - {`full`, `valid`, `same`}, optional. defaults to `full`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3.]);
    /// let other = Array::flat(vec![0., 1., 0.5]);
    /// assert_eq!(Array::flat(vec![0., 1., 2.5, 4., 1.5]), arr.convolve(&other.unwrap(), Some("full")));
    /// ```
    fn convolve(&self, other: &Array<N>, mode: Option<impl ConvolveModeType>) -> Result<Array<N>, ArrayError>;

    /// Clip (limit) the values in an array
    ///
    /// # Arguments
    ///
    /// * `a_min` - minimum array value
    /// * `a_max` - maximum array value
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// let a_min = Array::single(2.).unwrap();
    /// let a_max = Array::single(3.).unwrap();
    /// assert_eq!(Array::flat(vec![2., 2., 3., 3.]), arr.clip(Some(a_min), Some(a_max)));
    /// ```
    fn clip(&self, a_min: Option<Array<N>>, a_max: Option<Array<N>>) -> Result<Array<N>, ArrayError>;

    /// Computes square root of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 4, 9, 16]);
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]), arr.sqrt());
    /// ```
    fn sqrt(&self) -> Result<Array<N>, ArrayError>;

    /// Computes cube root of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 8, 27, 64]);
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]), arr.cbrt());
    /// ```
    fn cbrt(&self) -> Result<Array<N>, ArrayError>;

    /// Return the element-wise square of the input
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]), arr.square());
    /// ```
    fn square(&self) -> Result<Array<N>, ArrayError>;

    /// Computes absolute value of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, -2, 3, -4]);
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]), arr.absolute());
    /// ```
    fn absolute(&self) -> Result<Array<N>, ArrayError>;

    /// Computes absolute value of array elements
    /// alias on `absolute`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, -2, 3, -4]);
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]), arr.abs());
    /// ```
    fn abs(&self) -> Result<Array<N>, ArrayError>;

    /// Computes absolute value of array elements
    /// alias on `absolute`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, -2, 3, -4]);
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]), arr.fabs());
    /// ```
    fn fabs(&self) -> Result<Array<N>, ArrayError>;

    /// Returns an element-wise indication of the sign of a number
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, -2, -3, 4]);
    /// assert_eq!(Array::flat(vec![1, -1, -1, 1]), arr.sign());
    /// ```
    fn sign(&self) -> Result<Array<isize>, ArrayError>;

    /// Compute the Heaviside step function
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
    /// let arr = Array::flat(vec![-1.5, 0., 2.]);
    /// assert_eq!(Array::flat(vec![0., 0.5, 1.]), arr.heaviside(&Array::single(0.5).unwrap()));
    /// ```
    fn heaviside(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Replace NaN with zero and infinity with large finite numbers
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., f64::NAN, f64::INFINITY]);
    /// assert_eq!(Array::flat(vec![1., 2., 0., f64::MAX]), arr.nan_to_num());
    /// ```
    fn nan_to_num(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayMathMisc<N> for Array<N> {

    fn convolve(&self, other: &Array<N>, mode: Option<impl ConvolveModeType>) -> Result<Array<N>, ArrayError> {
        if self.len()? == 0 || other.len()? == 0 {
            return Err(ArrayError::ParameterError { param: "`array|other`", message: "cannot be empty", })
        }

        let mode = match mode {
            Some(cm) => cm.to_mode()?,
            None => ConvolveMode::Full,
        };

        let mut arrays = (self.to_array_f64()?, other.to_array_f64()?);
        if arrays.1.len()? > arrays.0.len()? { arrays = arrays.swap() };
        let mut out = vec![0.; arrays.0.len()? + arrays.1.len()? - 1];
        for i in 0 .. arrays.0.len()? { for j in 0 .. arrays.1.len()? {
            out[i + j] += arrays.0[i] * arrays.1[j]
        } }
        let (n, m) = (arrays.0.len()?, arrays.1.len()?);

        match mode {
            ConvolveMode::Full => out.to_vec(),
            ConvolveMode::Valid => out.iter().skip(m - 1).take(n - m + 1).copied().collect(),
            ConvolveMode::Same => out.iter().skip((m - 1) / 2).take(n).copied().collect(),
        }.to_array()?.to_array_num()
    }

    fn clip(&self, a_min: Option<Array<N>>, a_max: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        let a_min = if let Some(min) = a_min { min } else { self.min(None)? }
            .broadcast_to(self.get_shape()?)?;
        let a_max = if let Some(max) = a_max { max } else { self.max(None)? }
            .broadcast_to(self.get_shape()?)?;
        let borders = a_min.zip(&a_max)?;

        self.zip(&borders)?
            .map(|tuple| {
                if tuple.0 < tuple.1.0 { tuple.1.0 }
                else if tuple.0 > tuple.1.1 { tuple.1.1 }
                else { tuple.0 }
            })
    }

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sqrt()))
    }

    fn cbrt(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().cbrt()))
    }

    fn square(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().powf(2.)))
    }

    fn absolute(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().abs()))
    }

    fn abs(&self) -> Result<Array<N>, ArrayError> {
        self.absolute()
    }

    fn fabs(&self) -> Result<Array<N>, ArrayError> {
        self.absolute()
    }

    fn sign(&self) -> Result<Array<isize>, ArrayError> {
        self.map(|&i| if i < N::zero() { -1 } else { 1 })
    }

    fn heaviside(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.zip(other)?
            .map(|tuple|
                if tuple.0 < N::zero() { N::zero() }
                else if tuple.0 == N::zero() { tuple.1 }
                else { N::one() }
            )
    }

    fn nan_to_num(&self) -> Result<Array<N>, ArrayError> {
        self.map(|&item|
            if item.is_nan() { N::zero() }
            else if item.is_inf() { item.max() }
            else { item }
        )
    }
}

impl <N: Numeric> ArrayMathMisc<N> for Result<Array<N>, ArrayError> {

    fn convolve(&self, other: &Array<N>, mode: Option<impl ConvolveModeType>) -> Result<Array<N>, ArrayError> {
        self.clone()?.convolve(other, mode)
    }

    fn clip(&self, a_min: Option<Array<N>>, a_max: Option<Array<N>>) -> Result<Array<N>, ArrayError> {
        self.clone()?.clip(a_min, a_max)
    }

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sqrt()
    }

    fn cbrt(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cbrt()
    }

    fn square(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.square()
    }

    fn absolute(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.absolute()
    }

    fn abs(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.abs()
    }

    fn fabs(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.fabs()
    }

    fn sign(&self) -> Result<Array<isize>, ArrayError> {
        self.clone()?.sign()
    }

    fn heaviside(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.heaviside(other)
    }

    fn nan_to_num(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.nan_to_num()
    }
}
