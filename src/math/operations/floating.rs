use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    numeric::prelude::*,
};

/// `ArrayTrait` - Array Floating functions
pub trait ArrayFloating<N: Floating> where Self: Sized + Clone {

    /// Returns element-wise True where signbit is set (less than zero)
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., -2., -3., 4.]);
    /// assert_eq!(Array::flat(vec![false, true, true, false]), arr.signbit());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn signbit(&self) -> Result<Array<bool>, ArrayError>;

    /// Change the sign of x1 to that of x2, element-wise
    ///
    /// # Arguments
    ///
    /// * `other` - array to copy sign from
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., -2., -3., 4.]);
    /// let other = Array::flat(vec![-1., 2., -3., 4.]);
    /// assert_eq!(Array::flat(vec![-1., 2., -3., 4.]), arr.copysign(&other.unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn copysign(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Decompose the elements of x into man and twos exp
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::arange(0., 8., None);
    /// let result = arr.frexp().unwrap();
    /// assert_eq!(Array::flat(vec![0., 0.5, 0.5, 0.75, 0.5, 0.625, 0.75, 0.875, 0.5]).unwrap(), result.0);
    /// assert_eq!(Array::flat(vec![0, 1, 2, 2, 3, 3, 3, 3, 4]).unwrap(), result.1);
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn frexp(&self) -> Result<(Array<N>, Array<i32>), ArrayError>;

    /// Returns x1 * 2**x2, element-wise. Inverse of frexp
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![0., 0.5, 0.5, 0.75, 0.5, 0.625, 0.75, 0.875, 0.5]);
    /// let other = Array::flat(vec![0, 1, 2, 2, 3, 3, 3, 3, 4]);
    /// assert_eq!(Array::arange(0., 8., None), arr.ldexp(&other.unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn ldexp(&self, other: &Array<i32>) -> Result<Array<N>, ArrayError>;

    /// Return the next floating-point value after x1 towards x2, element-wise
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected = Array::flat(vec![1. + f64::EPSILON, 2. - f64::EPSILON]);
    /// assert_eq!(expected, Array::flat(vec![1., 2.]).nextafter(&Array::flat(vec![2., 1.]).unwrap()));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn nextafter(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Return the distance between x and the nearest adjacent number
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(Array::flat(vec![f64::EPSILON, f64::EPSILON * 2.]), Array::flat(vec![1., 2.]).spacing());
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn spacing(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Floating> ArrayFloating<N> for Array<N> {

    fn signbit(&self) -> Result<Array<bool>, ArrayError> {
        self.map(|e| e.to_f64().is_sign_negative())
    }

    fn copysign(&self, other: &Self) -> Result<Self, ArrayError> {
        self.zip(other)?
            .map(|item| N::from(item.0.to_f64().copysign(item.1.to_f64())))
    }

    fn frexp(&self) -> Result<(Self, Array<i32>), ArrayError> {

        fn _frexp(x: f64) -> (f64, i32) {
            let sign = x.signum();
            let mut x = x.abs();
            let mut sig: f64 = 0.0;
            let mut exp: i32 = 0;
            if x == 0.0 { return (sig, exp); }

            while x >= 1.0 { x /= 2.0; exp += 1; }
            while x < 0.5 { x *= 2.0; exp -= 1; }

            sig = x;
            (sig * sign, exp)
        }

        let mut man = vec![];
        let mut exp = vec![];

        self.for_each(|item| {
            let result = _frexp(item.to_f64());
            man.push(N::from(result.0));
            exp.push(result.1);
        })?;

        Ok((
            man.to_array().reshape(&self.get_shape()?)?,
            exp.to_array().reshape(&self.get_shape()?)?,
        ))
    }

    fn ldexp(&self, other: &Array<i32>) -> Result<Self, ArrayError> {

        fn _ldexp(x: f64, exp: i32) -> f64 {
            if x == 0. { return x }
            let mut exp = exp;
            let mut sig = x;

            while exp > 0 { sig *= 2.; exp -= 1; }
            while exp < 0 { sig /= 2.; exp += 1; }

            sig
        }

        self.zip(other)?
            .map(|item| N::from(_ldexp(item.0.to_f64(), item.1)))
    }

    fn nextafter(&self, other: &Self) -> Result<Self, ArrayError> {

        fn _nextafter(x: f64, y: f64) -> f64 {
            if (x - y).abs() < 1e-24 { x }
            else if x < y { x + f64::EPSILON }
            else { x - f64::EPSILON }
        }

        self.zip(other)?
            .map(|item| N::from(_nextafter(item.0.to_f64(), item.1.to_f64())))
    }

    fn spacing(&self) -> Result<Self, ArrayError> {

        fn _spacing(x: f64) -> f64 {
            let bits = x.to_bits();
            let next =
                if x.is_sign_negative() { bits - 1 }
                else { bits + 1 };
            f64::from_bits(next) - x
        }

        self.map(|item| N::from(_spacing(item.to_f64())))
    }
}

impl <N: Floating> ArrayFloating<N> for Result<Array<N>, ArrayError> {

    fn signbit(&self) -> Result<Array<bool>, ArrayError> {
        self.clone()?.signbit()
    }

    fn copysign(&self, other: &Array<N>) -> Self {
        self.clone()?.copysign(other)
    }

    fn frexp(&self) -> Result<(Array<N>, Array<i32>), ArrayError> {
        self.clone()?.frexp()
    }

    fn ldexp(&self, other: &Array<i32>) -> Self {
        self.clone()?.ldexp(other)
    }

    fn nextafter(&self, other: &Array<N>) -> Self {
        self.clone()?.nextafter(other)
    }

    fn spacing(&self) -> Self {
        self.clone()?.spacing()
    }
}
