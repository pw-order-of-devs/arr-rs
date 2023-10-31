use crate::{
    core::prelude::*,
    errors::prelude::*,
    math::prelude::*,
    numeric::prelude::*,
};

/// `ArrayTrait` - Array Arithmetic functions
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn subtract(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes remainder of division element-wise
    /// alias on `remainder`
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
    /// assert_eq!(Array::flat(vec![1, 0, 1, 0]), arr.r#mod(&Array::single(2).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn r#mod(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes remainder of division element-wise
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
    /// assert_eq!(Array::flat(vec![1, 0, 1, 0]), arr.fmod(&Array::single(2).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn fmod(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes fractional and integral parts of an array, element-wise
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1.5, 2., 3.5]);
    /// assert_eq!(Ok((Array::flat(vec![0.5, 0., 0.5]).unwrap(), Array::flat(vec![1., 2., 3.]).unwrap())), arr.modf());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn modf(&self) -> Result<(Array<N>, Array<N>), ArrayError>;

    /// Computes remainder of division element-wise
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
    /// assert_eq!(Array::flat(vec![1, 0, 1, 0]), arr.remainder(&Array::single(2).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn remainder(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes integral and fractional parts of an array, element-wise
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1.5, 2., 3.5]);
    /// assert_eq!(Ok((Array::flat(vec![1., 2., 3.]).unwrap(), Array::flat(vec![0.5, 0., 0.5]).unwrap())), arr.divmod());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn divmod(&self) -> Result<(Array<N>, Array<N>), ArrayError>;
}

impl <N: Numeric> ArrayArithmetic<N> for Array<N> {

    fn add(&self, value: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() + tuple.1.to_f64()))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn reciprocal(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().recip()))
    }

    fn positive(&self) -> Result<Self, ArrayError> {
        Ok(self.clone())
    }

    fn negative(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(-i.to_f64()))
    }

    fn multiply(&self, value: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() * tuple.1.to_f64()))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn divide(&self, value: &Self) -> Result<Self, ArrayError> {
        if value.get_elements()?.contains(&N::zero()) {
            return Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`", });
        }
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() / tuple.1.to_f64()))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn true_divide(&self, value: &Self) -> Result<Self, ArrayError> {
        self.divide(value)
    }

    fn floor_divide(&self, value: &Self) -> Result<Self, ArrayError> {
        self.divide(value).floor()
    }

    fn power(&self, value: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().powi(tuple.1.to_i32())))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn float_power(&self, value: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().powf(tuple.1.to_f64())))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn subtract(&self, value: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() - tuple.1.to_f64()))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn r#mod(&self, value: &Self) -> Result<Self, ArrayError> {
        self.remainder(value)
    }

    fn fmod(&self, value: &Self) -> Result<Self, ArrayError> {
        if value.get_elements()?.contains(&N::zero()) {
            return Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`", });
        }
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from((tuple.0.to_f64() / tuple.1.to_f64()).floor().mul_add(-tuple.1.to_f64(), tuple.0.to_f64())))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn modf(&self) -> Result<(Self, Self), ArrayError> {
        let fractional = self.r#mod(&Self::single(N::one())?)?;
        let integral = self.floor()?;
        Ok((fractional, integral))
    }

    fn remainder(&self, value: &Self) -> Result<Self, ArrayError> {
        if value.get_elements()?.contains(&N::zero()) {
            return Err(ArrayError::ParameterError { param: "value", message: "cannot contain `0`", });
        }
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64() % tuple.1.to_f64()))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn divmod(&self) -> Result<(Self, Self), ArrayError> {
        let fractional = self.r#mod(&Self::single(N::one())?)?;
        let integral = self.floor()?;
        Ok((integral, fractional))
    }
}

impl <N: Numeric> ArrayArithmetic<N> for Result<Array<N>, ArrayError> {

    fn add(&self, value: &Array<N>) -> Self {
        self.clone()?.add(value)
    }

    fn reciprocal(&self) -> Self {
        self.clone()?.reciprocal()
    }

    fn positive(&self) -> Self {
        self.clone()?.positive()
    }

    fn negative(&self) -> Self {
        self.clone()?.negative()
    }

    fn multiply(&self, value: &Array<N>) -> Self {
        self.clone()?.multiply(value)
    }

    fn divide(&self, value: &Array<N>) -> Self {
        self.clone()?.divide(value)
    }

    fn true_divide(&self, value: &Array<N>) -> Self {
        self.clone()?.true_divide(value)
    }

    fn floor_divide(&self, value: &Array<N>) -> Self {
        self.clone()?.floor_divide(value)
    }

    fn power(&self, value: &Array<N>) -> Self {
        self.clone()?.power(value)
    }

    fn float_power(&self, value: &Array<N>) -> Self {
        self.clone()?.float_power(value)
    }

    fn subtract(&self, value: &Array<N>) -> Self {
        self.clone()?.subtract(value)
    }

    fn r#mod(&self, value: &Array<N>) -> Self {
        self.clone()?.r#mod(value)
    }

    fn fmod(&self, value: &Array<N>) -> Self {
        self.clone()?.fmod(value)
    }

    fn modf(&self) -> Result<(Array<N>, Array<N>), ArrayError> {
        self.clone()?.modf()
    }

    fn remainder(&self, value: &Array<N>) -> Self {
        self.clone()?.remainder(value)
    }

    fn divmod(&self) -> Result<(Array<N>, Array<N>), ArrayError> {
        self.clone()?.divmod()
    }
}
