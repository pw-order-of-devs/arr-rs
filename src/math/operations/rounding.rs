use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Rounding functions
pub trait ArrayRounding<N: Numeric> where Self: Sized + Clone {

    /// Evenly round to the given number of decimals
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places to round to (default: 0). If decimals is negative, it specifies the number of positions to the left of the decimal point
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, 8.0010, 22.234]);
    /// assert_eq!(Array::flat(vec![2., 4.6, 8.001, 20.]), arr.round(&Array::flat(vec![0, 1, 3, -1]).unwrap()));
    /// ```
    fn round(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError>;

    /// Evenly round to the given number of decimals. alias on `round`
    ///
    /// # Arguments
    ///
    /// * `decimals` - Number of decimal places to round to (default: 0). If decimals is negative, it specifies the number of positions to the left of the decimal point
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, 8.0010, 22.234]);
    /// assert_eq!(Array::flat(vec![2., 4.6, 8.001, 20.]), arr.around(&Array::flat(vec![0, 1, 3, -1]).unwrap()));
    /// ```
    fn around(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError>;

    /// Round elements of the array to the nearest integer
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, 8.0010, 22.234]);
    /// assert_eq!(Array::flat(vec![2., 5., 8., 22.]), arr.rint());
    /// ```
    fn rint(&self) -> Result<Array<N>, ArrayError>;

    /// Round to nearest integer towards zero
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, -1.6, -2.2]);
    /// assert_eq!(Array::flat(vec![2., 4., -1., -2.]), arr.fix());
    /// ```
    fn fix(&self) -> Result<Array<N>, ArrayError>;

    /// Round to nearest integer towards zero
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, -1.6, -2.2]);
    /// assert_eq!(Array::flat(vec![2., 4., -1., -2.]), arr.fix());
    /// ```
    fn trunc(&self) -> Result<Array<N>, ArrayError>;

    /// Return the floor of the input, element-wise
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, -1.6, -2.2]);
    /// assert_eq!(Array::flat(vec![2., 4., -2., -3.]), arr.floor());
    /// ```
    fn floor(&self) -> Result<Array<N>, ArrayError>;

    /// Return the ceil of the input, element-wise
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2.01, 4.6, -1.6, -2.2]);
    /// assert_eq!(Array::flat(vec![3., 5., -1., -2.]), arr.ceil());
    /// ```
    fn ceil(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayRounding<N> for Array<N> {

    fn round(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError> {
        let (array, other) = self.broadcast_h2(decimals)?;
        let elements = array.clone().into_iter().zip(&other)
            .map(|tuple| {
                let multiplier = 10_f64.powi(*tuple.1 as i32);
                N::from((tuple.0.to_f64() * multiplier).round() / multiplier)
            })
            .collect();
        Array::new(elements, array.get_shape()?)
    }

    fn around(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError> {
        self.round(decimals)
    }

    fn rint(&self) -> Result<Array<N>, ArrayError> {
        self.round(&Array::single(0).unwrap())
    }

    fn fix(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i|
            if *i >= N::zero() { N::from(i.to_f64().floor()) }
            else { N::from(i.to_f64().ceil()) }
        )
    }

    fn trunc(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().trunc()))
    }

    fn floor(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().floor()))
    }

    fn ceil(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().ceil()))
    }
}

impl <N: Numeric> ArrayRounding<N> for Result<Array<N>, ArrayError> {

    fn round(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.round(decimals)
    }

    fn around(&self, decimals: &Array<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.around(decimals)
    }

    fn rint(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.rint()
    }

    fn fix(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.fix()
    }

    fn trunc(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.trunc()
    }

    fn floor(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.floor()
    }

    fn ceil(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.ceil()
    }
}
