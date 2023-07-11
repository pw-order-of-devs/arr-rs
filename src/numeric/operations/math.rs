use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array Math functions
pub trait ArrayMath<N: Numeric> where Self: Sized + Clone {

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

    /// Multiplication of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// assert_eq!(24, arr.product().unwrap());
    /// ```
    fn product(&self) -> Result<N, ArrayError>;

    /// Sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]);
    /// assert_eq!(10, arr.sum().unwrap());
    /// ```
    fn sum(&self) -> Result<N, ArrayError>;

    /// Cumulative sum of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1,2,3,4]).unwrap();
    /// assert_eq!(Array::flat(vec![1, 3, 6, 10]), arr.cumsum());
    /// ```
    fn cumsum(&self) -> Result<Array<N>, ArrayError>;

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
    fn atanh(&self) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayMath<N> for Array<N> {

    fn sqrt(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sqrt()))
            .reshape(self.get_shape()?)
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

    fn product(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::one(), |acc, x| N::from(acc.to_f64() * x.to_f64())))
    }

    fn sum(&self) -> Result<N, ArrayError> {
        Ok(self.elements.iter().fold(N::zero(), |acc, x| N::from(acc.to_f64() + x.to_f64())))
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        let mut acc = N::zero();
        self.map(|&x| {
            acc = N::from(acc.to_f64() + x.to_f64());
            acc
        })
    }

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

    fn sinh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().sinh()))
    }

    fn cosh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().cosh()))
    }

    fn tanh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().tanh()))
    }

    fn asinh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().asinh()))
    }

    fn acosh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().acosh()))
    }

    fn atanh(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().atanh()))
    }
}

impl <N: Numeric> ArrayMath<N> for Result<Array<N>, ArrayError> {

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

    fn product(&self) -> Result<N, ArrayError> {
        self.clone()?.product()
    }

    fn sum(&self) -> Result<N, ArrayError> {
        self.clone()?.sum()
    }

    fn cumsum(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cumsum()
    }

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

    fn sinh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.sinh()
    }

    fn cosh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.cosh()
    }

    fn tanh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.tanh()
    }

    fn asinh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.asinh()
    }

    fn acosh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.acosh()
    }

    fn atanh(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.atanh()
    }
}

