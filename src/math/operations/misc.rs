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

    /// Computes sqrt of array elements
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

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sqrt()))
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
}

impl <N: Numeric> ArrayMathMisc<N> for Result<Array<N>, ArrayError> {

    fn convolve(&self, other: &Array<N>, mode: Option<impl ConvolveModeType>) -> Result<Array<N>, ArrayError> {
        self.clone()?.convolve(other, mode)
    }

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sqrt()
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
}
