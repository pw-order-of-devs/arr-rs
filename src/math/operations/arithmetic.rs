use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};
use crate::prelude::ArrayRounding;

/// ArrayTrait - Array Math functions
pub trait ArrayArithmetic<N: Numeric> where Self: Sized + Clone {

    /// Add arguments element-wise
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![3, 4, 5, 6]), arr.add(&Array::single(2).unwrap()));
    /// ```
    fn add(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes reciprocal of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 4., 10.]);
    /// assert_eq!(Array::flat(vec![1., 0.5, 0.25, 0.1]), arr.reciprocal());
    /// ```
    fn reciprocal(&self) -> Result<Array<N>, ArrayError>;

    /// Computes numerical positive of array elements
    /// Equivalent to `self.clone()`
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., -1.]);
    /// assert_eq!(Array::flat(vec![1., -1.]), arr.positive());
    /// ```
    fn positive(&self) -> Result<Array<N>, ArrayError>;

    /// Computes numerical negative of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., -1.]);
    /// assert_eq!(Array::flat(vec![-1., 1.]), arr.negative());
    /// ```
    fn negative(&self) -> Result<Array<N>, ArrayError>;

    /// Multiply arguments element-wise
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![2, 4, 6, 8]), arr.multiply(&Array::single(2).unwrap()));
    /// ```
    fn multiply(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Divide arguments element-wise
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![0.5, 1., 1.5, 2.]), arr.divide(&Array::single(2.).unwrap()));
    /// ```
    fn divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Divide arguments element-wise
    /// alias on `divide`
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![0.5, 1., 1.5, 2.]), arr.true_divide(&Array::single(2.).unwrap()));
    /// ```
    fn true_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Divide arguments element-wise, returning floor value
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![0., 1., 1., 2.]), arr.floor_divide(&Array::single(2.).unwrap()));
    /// ```
    fn floor_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes integer power of array elements
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]), arr.power(&Array::single(2).unwrap()));
    /// ```
    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes float power of array elements
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![1, 4, 9, 16]), arr.float_power(&Array::single(2).unwrap()));
    /// ```
    fn float_power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Subtract arguments element-wise
    ///
    /// # Arguments
    ///
    /// * `value` - other array to perform operations on
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1, 2, 3, 4]);
    /// assert_eq!(Array::flat(vec![-1, 0, 1, 2]), arr.subtract(&Array::single(2).unwrap()));
    /// ```
    fn subtract(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayArithmetic<N> for Array<N> {

    fn add(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() + tuple.1.to_f64()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn reciprocal(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().recip()))
    }

    fn positive(&self) -> Result<Array<N>, ArrayError> {
        Ok(self.clone())
    }

    fn negative(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(-i.to_f64()))
    }

    fn multiply(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() * tuple.1.to_f64()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() / tuple.1.to_f64()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn true_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.divide(value)
    }

    fn floor_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.divide(value).floor()
    }

    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().powi(tuple.1.to_i32())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn float_power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().powf(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn subtract(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() - tuple.1.to_f64()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }
}

impl <N: Numeric> ArrayArithmetic<N> for Result<Array<N>, ArrayError> {

    fn add(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.add(value)
    }

    fn reciprocal(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.reciprocal()
    }

    fn positive(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.positive()
    }

    fn negative(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.negative()
    }

    fn multiply(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.multiply(value)
    }

    fn divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.divide(value)
    }

    fn true_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.true_divide(value)
    }

    fn floor_divide(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.floor_divide(value)
    }

    fn power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.power(value)
    }

    fn float_power(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.float_power(value)
    }

    fn subtract(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.subtract(value)
    }
}
