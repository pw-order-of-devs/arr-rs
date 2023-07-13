use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Trigonometric functions
pub trait ArrayTrigonometric<N: Numeric> where Self: Sized + Clone {

    /// Compute the sine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-0.8414709848078965, 0., 0.8414709848078965]), arr.sin());
    /// ```
    fn sin(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the cosine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![0.5403023058681398, 1., 0.5403023058681398]), arr.cos());
    /// ```
    fn cos(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the tangent of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-1.5574077246549023, 0., 1.5574077246549023]), arr.tan());
    /// ```
    fn tan(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse sine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-std::f64::consts::FRAC_PI_2, 0., std::f64::consts::FRAC_PI_2]), arr.asin());
    /// ```
    fn asin(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse cosine of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![std::f64::consts::PI, std::f64::consts::FRAC_PI_2, 0.]), arr.acos());
    /// ```
    fn acos(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse tangent of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// assert_eq!(Array::flat(vec![-std::f64::consts::FRAC_PI_4, 0., std::f64::consts::FRAC_PI_4]), arr.atan());
    /// ```
    fn atan(&self) -> Result<Array<N>, ArrayError>;

    /// Compute the inverse tangent of x1/x2 choosing the quadrant correctly
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
    /// let arr = Array::flat(vec![-1., 0., 1.]).unwrap();
    /// let other = Array::flat(vec![2., 4., 6.]).unwrap();
    /// assert_eq!(Array::flat(vec![-0.4636476090008061,0., 0.16514867741462683]), arr.atan2(&other));
    /// ```
    fn atan2(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Given the “legs” of a right triangle, return its hypotenuse
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
    /// let arr = Array::full(vec![3, 3], 3).unwrap();
    /// let other = Array::full(vec![3, 3], 4).unwrap();
    /// assert_eq!(Array::full(vec![3, 3], 5), arr.hypot(&other));
    /// ```
    fn hypot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Convert angles from radians to degrees
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![std::f64::consts::FRAC_PI_6, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]).unwrap();
    /// let expected = Array::flat(vec![30., 45., 60., 90.]);
    /// assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", arr.degrees()));
    /// ```
    fn degrees(&self) -> Result<Array<N>, ArrayError>;

    /// Convert angles from radians to degrees. alias on `degrees`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![std::f64::consts::FRAC_PI_6, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]).unwrap();
    /// let expected = Array::flat(vec![30., 45., 60., 90.]);
    /// assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", arr.rad2deg()));
    /// ```
    fn rad2deg(&self) -> Result<Array<N>, ArrayError>;

    /// Convert angles from degrees to radians
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![30., 45., 60., 90.]).unwrap();
    /// let expected = Array::flat(vec![std::f64::consts::FRAC_PI_6, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]);
    /// assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", arr.radians()));
    /// ```
    fn radians(&self) -> Result<Array<N>, ArrayError>;

    /// Convert angles from degrees to radians. alias on `radians`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![30., 45., 60., 90.]).unwrap();
    /// let expected = Array::flat(vec![std::f64::consts::FRAC_PI_6, std::f64::consts::FRAC_PI_4, std::f64::consts::FRAC_PI_3, std::f64::consts::FRAC_PI_2]);
    /// assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", arr.deg2rad()));
    /// ```
    fn deg2rad(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayTrigonometric<N> for Array<N> {

    fn sin(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sin()))
    }

    fn cos(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().cos()))
    }

    fn tan(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().tan()))
    }

    fn asin(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().asin()))
    }

    fn acos(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().acos()))
    }

    fn atan(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().atan()))
    }

    fn atan2(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().atan2(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn hypot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().hypot(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn degrees(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().to_degrees()))
    }

    fn rad2deg(&self) -> Result<Array<N>, ArrayError> {
        self.degrees()
    }

    fn radians(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().to_radians()))
    }

    fn deg2rad(&self) -> Result<Array<N>, ArrayError> {
        self.radians()
    }
}

impl <N: Numeric> ArrayTrigonometric<N> for Result<Array<N>, ArrayError> {

    fn sin(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sin()
    }

    fn cos(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cos()
    }

    fn tan(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.tan()
    }

    fn asin(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.asin()
    }

    fn acos(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.acos()
    }

    fn atan(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.atan()
    }

    fn atan2(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.atan2(other)
    }

    fn hypot(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.hypot(other)
    }

    fn degrees(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.degrees()
    }

    fn rad2deg(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.rad2deg()
    }

    fn radians(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.radians()
    }

    fn deg2rad(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.deg2rad()
    }
}
