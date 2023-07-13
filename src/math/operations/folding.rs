use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
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
    /// assert_eq!(24, arr.prod().unwrap());
    /// ```
    fn prod(&self) -> Result<N, ArrayError>;

    /// Sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(10, arr.sum().unwrap());
    /// ```
    fn sum(&self) -> Result<N, ArrayError>;

    /// Sum of array elements treating NaN as one
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(24., arr.nanprod().unwrap());
    /// ```
    fn nanprod(&self) -> Result<N, ArrayError>;

    /// Sum of array elements treating NaN as zero
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(10., arr.nansum().unwrap());
    /// ```
    fn nansum(&self) -> Result<N, ArrayError>;

    /// Cumulative product of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 2, 6, 24]), arr.cumprod());
    /// ```
    fn cumprod(&self) -> Result<Array<N>, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]), arr.cumsum());
    /// ```
    fn cumsum(&self) -> Result<Array<N>, ArrayError>;

    /// Cumulative product of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(Array::flat(vec![1., 2., 6., 24., 24.]), arr.nancumprod());
    /// ```
    fn nancumprod(&self) -> Result<Array<N>, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4., f64::NAN]).unwrap();
    /// assert_eq!(Array::flat(vec![1., 3., 6., 10., 10.]), arr.nancumsum());
    /// ```
    fn nancumsum(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayFolding<N> for Array<N> {

    fn prod(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::one(), |acc, x| N::from(acc.to_f64() * x.to_f64())))
    }

    fn sum(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::zero(), |acc, x| N::from(acc.to_f64() + x.to_f64())))
    }

    fn nanprod(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::one(), |acc, x| {
            let x = if x.to_f64().is_nan() { 1. } else { x.to_f64() };
            N::from(acc.to_f64() * x)
        }))
    }

    fn nansum(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::zero(), |acc, x| {
            println!("{acc} {x}");
            let x = if x.to_f64().is_nan() { 0. } else { x.to_f64() };
            N::from(acc.to_f64() + x)
        }))
    }

    fn cumprod(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::one();
        self.map(|&x| {
            acc = N::from(acc.to_f64() * x.to_f64());
            acc
        })
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::zero();
        self.map(|&x| {
            acc = N::from(acc.to_f64() + x.to_f64());
            acc
        })
    }

    fn nancumprod(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::one();
        self.map(|&x| {
            let x = if x.to_f64().is_nan() { 1. } else { x.to_f64() };
            acc = N::from(acc.to_f64() * x);
            acc
        })
    }

    fn nancumsum(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::zero();
        self.map(|&x| {
            let x = if x.to_f64().is_nan() { 0. } else { x.to_f64() };
            acc = N::from(acc.to_f64() + x);
            acc
        })
    }
}

impl <N: Numeric> ArrayFolding<N> for Result<Array<N>, ArrayError> {

    fn prod(&self) -> Result<N, ArrayError> {
        self.clone()?.prod()
    }

    fn sum(&self) -> Result<N, ArrayError> {
        self.clone()?.sum()
    }

    fn nanprod(&self) -> Result<N, ArrayError> {
        self.clone()?.nanprod()
    }

    fn nansum(&self) -> Result<N, ArrayError> {
        self.clone()?.nansum()
    }

    fn cumprod(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumprod()
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumsum()
    }

    fn nancumprod(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.nancumprod()
    }

    fn nancumsum(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.nancumsum()
    }
}
