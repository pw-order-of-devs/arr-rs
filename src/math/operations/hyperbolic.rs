use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// `ArrayTrait` - Array Hyperbolic functions
pub trait ArrayHyperbolic<N: Numeric> where Self: Sized + Clone {

    /// Compute the hyperbolic sine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-1.1752011936438014, 0., 1.1752011936438014]), arr.sinh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn sinh(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the hyperbolic cosine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![1.5430806348152437, 1., 1.5430806348152437]), arr.cosh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn cosh(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the hyperbolic tangent of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-0.7615941559557649, 0., 0.7615941559557649]), arr.tanh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn tanh(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse hyperbolic sine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-0.881373587019543, 0., 0.881373587019543]), arr.asinh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn asinh(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse hyperbolic cosine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3.]).unwrap();
    /// assert_eq!(Array::flat(vec![0., 1.3169578969248166, 1.762747174039086]), arr.acosh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn acosh(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse hyperbolic tangent of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-f64::INFINITY, 0., f64::INFINITY]), arr.atanh());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn atanh(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayHyperbolic<N> for Array<N> {
    
    fn sinh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().sinh()))
    }

    fn cosh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().cosh()))
    }

    fn tanh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().tanh()))
    }

    fn asinh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().asinh()))
    }

    fn acosh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().acosh()))
    }

    fn atanh(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().atanh()))
    }
}

impl <N: Numeric> ArrayHyperbolic<N> for Result<Array<N>, ArrayError> {

    fn sinh(&self) -> Self {
        self.clone()?.sinh()
    }

    fn cosh(&self) -> Self {
        self.clone()?.cosh()
    }

    fn tanh(&self) -> Self {
        self.clone()?.tanh()
    }

    fn asinh(&self) -> Self {
        self.clone()?.asinh()
    }

    fn acosh(&self) -> Self {
        self.clone()?.acosh()
    }

    fn atanh(&self) -> Self {
        self.clone()?.atanh()
    }
}
