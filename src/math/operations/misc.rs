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

    /// Computes natural logarithm of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 4., 8., 16.]).unwrap();
    /// assert_eq!(Array::flat(vec![0., 1.3862943611198906, 2.0794415416798357, 2.772588722239781]).unwrap(), arr.log().unwrap());
    /// ```
    fn log(&self) -> Result<Array<N>, ArrayError>;

    /// Computes logarithm base 2 of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 4., 8., 16.]).unwrap();
    /// assert_eq!(Array::flat(vec![0., 2., 3., 4.]).unwrap(), arr.log2().unwrap());
    /// ```
    fn log2(&self) -> Result<Array<N>, ArrayError>;

    /// Computes logarithm base 10 of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 10., 100.]).unwrap();
    /// assert_eq!(Array::flat(vec![0., 1., 2.]).unwrap(), arr.log10().unwrap());
    /// ```
    fn log10(&self) -> Result<Array<N>, ArrayError>;

    /// Computes logarithm base n of array elements
    ///
    /// # Arguments
    ///
    /// * `value` - log array to perform the operation with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2., 4., 8., 20.]).unwrap();
    /// assert_eq!(Array::flat(vec![1., 2., 3., 1.301029995663981]).unwrap(), arr.logn(&Array::flat(vec![2., 2., 2., 10.]).unwrap()).unwrap());
    /// ```
    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes power of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]).unwrap(), arr.power(&Array::single(2).unwrap()).unwrap());
    /// ```
    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayMathMisc<N> for Array<N> {

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sqrt()))
    }

    fn log(&self) -> Result<Array<N>, ArrayError> {
        self.logn(&Array::single(N::from(std::f64::consts::E)).unwrap())
    }

    fn log2(&self) -> Result<Array<N>, ArrayError> {
        self.logn(&Array::single(N::from(2)).unwrap())
    }

    fn log10(&self) -> Result<Array<N>, ArrayError> {
        self.logn(&Array::single(N::from(10)).unwrap())
    }

    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().log(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().powf(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }
}

impl <N: Numeric> ArrayMathMisc<N> for Result<Array<N>, ArrayError> {

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sqrt()
    }

    fn log(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.log()
    }

    fn log2(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.log2()
    }

    fn log10(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.log10()
    }

    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.logn(value)
    }

    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.power(value)
    }
}
