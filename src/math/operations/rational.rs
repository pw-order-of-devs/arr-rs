use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};
use crate::prelude::ArrayMathMisc;

/// ArrayTrait - Array Rational functions
pub trait ArrayRational<N: Numeric> where Self: Sized + Clone {

    /// Returns the lowest common multiple of |x1| and |x2|
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
    /// assert_eq!(Array::single(60), Array::single(12).lcm(&Array::single(20).unwrap()));
    /// ```
    fn lcm(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Returns the greatest common divisor of |x1| and |x2|
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
    /// assert_eq!(Array::single(4), Array::single(12).gcd(&Array::single(20).unwrap()));
    /// ```
    fn gcd(&self, other: &Array<N>) -> Result<Array<N>, ArrayError>;
}

fn _gcd(x: i32, y: i32) -> i32 {
    if y == 0 { x }
    else { _gcd(y, x % y) }
}

impl <N: Numeric> ArrayRational<N> for Array<N> {

    fn lcm(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.abs()?.zip(&other.abs()?)?
            .map(|item| {
                let (x, y) = (item.0.to_i32(), item.1.to_i32());
                N::from(x * y / _gcd(x, y))
            })
    }

    fn gcd(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.abs()?.zip(&other.abs()?)?
            .map(|item| N::from(_gcd(item.0.to_i32(), item.1.to_i32())))
    }
}

impl <N: Numeric> ArrayRational<N> for Result<Array<N>, ArrayError> {

    fn lcm(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.lcm(other)
    }

    fn gcd(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.gcd(other)
    }
}
