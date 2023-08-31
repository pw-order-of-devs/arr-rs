use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
};

/// ArrayTrait - Array ExpLog functions
pub trait ArrayExpLog<N: Numeric> where Self: Sized + Clone {

    /// Computes the exponential of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![std::f64::consts::E,  7.38905609893065, 20.085536923187668, 54.598150033144236]), arr.exp());
    /// ```
    fn exp(&self) -> Result<Array<N>, ArrayError>;

    /// Computes 2**element of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![2., 4., 8., 16.]), arr.exp2());
    /// ```
    fn exp2(&self) -> Result<Array<N>, ArrayError>;

    /// Computes exp - 1 of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 2., 3., 4.]);
    /// assert_eq!(Array::flat(vec![1.718281828459045, 6.38905609893065, 19.085536923187668, 53.598150033144236]), arr.exp_m1());
    /// ```
    fn exp_m1(&self) -> Result<Array<N>, ArrayError>;

    /// Computes natural logarithm of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 4., 8., 16.]);
    /// assert_eq!(Array::flat(vec![0., 1.3862943611198906, 2.0794415416798357, 2.772588722239781]), arr.log());
    /// ```
    fn log(&self) -> Result<Array<N>, ArrayError>;

    /// Computes logarithm base 2 of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 4., 8., 16.]);
    /// assert_eq!(Array::flat(vec![0., 2., 3., 4.]), arr.log2());
    /// ```
    fn log2(&self) -> Result<Array<N>, ArrayError>;

    /// Computes logarithm base 10 of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![1., 10., 100.]);
    /// assert_eq!(Array::flat(vec![0., 1., 2.]), arr.log10());
    /// ```
    fn log10(&self) -> Result<Array<N>, ArrayError>;

    /// Computes log(1 + x) of array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![2., 4., 8., 20.]);
    /// assert_eq!(Array::flat(vec![1.0986122886681098, 1.6094379124341003, 2.1972245773362196, 3.044522437723423]), arr.log_1p());
    /// ```
    fn log_1p(&self) -> Result<Array<N>, ArrayError>;

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
    /// let arr = Array::flat(vec![2., 4., 8., 20.]);
    /// assert_eq!(Array::flat(vec![1., 2., 3., 1.301029995663981]), arr.logn(&Array::flat(vec![2., 2., 2., 10.]).unwrap()));
    /// ```
    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes log(exp(x1) + exp(x2)) of array elements
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
    /// let arr_1 = Array::flat(vec![2., 4., 8., 20.]);
    /// let arr_2 = Array::flat(vec![2., 2., 2., 10.]).unwrap();
    /// assert_eq!(Array::flat(vec![2.6931471805599454, 4.126928011042972, 8.00247568513773, 20.000045398899218]), arr_1.log_add_exp(&arr_2));
    /// ```
    fn log_add_exp(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;

    /// Computes log2(2**x1 + 2**x2) of array elements
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
    /// let arr_1 = Array::flat(vec![2., 4., 8., 20.]);
    /// let arr_2 = Array::flat(vec![2., 2., 2., 10.]).unwrap();
    /// assert_eq!(Array::flat(vec![3., 4.321928094887363, 6.087462841250339, 8.965784284662087]), arr_1.log_add_exp2(&arr_2));
    /// ```
    fn log_add_exp2(&self, value: &Array<N>) -> Result<Array<N>, ArrayError>;
}

impl <N: Numeric> ArrayExpLog<N> for Array<N> {

    fn exp(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().exp()))
    }

    fn exp2(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().exp2()))
    }

    fn exp_m1(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().exp_m1()))
    }

    fn log(&self) -> Result<Array<N>, ArrayError> {
        self.logn(&Array::single(N::from(std::f64::consts::E)).unwrap())
    }

    fn log2(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().log2()))
    }

    fn log10(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from(i.to_f64().log10()))
    }

    fn log_1p(&self) -> Result<Array<N>, ArrayError> {
        self.map(|i| N::from((i.to_f64() + 1.).log(std::f64::consts::E)))
    }

    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().log(tuple.1.to_f64())))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn log_add_exp(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from((tuple.0.to_f64().exp() + tuple.1.to_f64().exp()).log(std::f64::consts::E)))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }

    fn log_add_exp2(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        let broadcasted = self.broadcast(value)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from((tuple.0.to_f64().powf(2.) + tuple.1.to_f64().powf(2.)).log2()))
            .collect();
        Array::new(elements, broadcasted.get_shape()?)
    }
}

impl <N: Numeric> ArrayExpLog<N> for Result<Array<N>, ArrayError> {

    fn exp(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.exp()
    }

    fn exp2(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.exp2()
    }

    fn exp_m1(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.exp_m1()
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

    fn log_1p(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.log_1p()
    }

    fn logn(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.logn(value)
    }

    fn log_add_exp(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.log_add_exp(value)
    }

    fn log_add_exp2(&self, value: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.log_add_exp2(value)
    }
}
