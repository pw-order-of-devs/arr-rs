use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Math functions
pub trait ArrayMathMisc<N: Numeric> where Self: Sized + Clone {

    /// Computes sqrt of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 4, 9, 16]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 2, 3, 4]).unwrap(), arr.sqrt().unwrap());
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
