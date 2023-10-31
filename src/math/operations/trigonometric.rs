use crate::{
    core::prelude::*,
    errors::prelude::*,
    extensions::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// `ArrayTrait` - Array Trigonometric functions
pub trait ArrayTrigonometric<N: NumericOps> where Self: Sized + Clone {

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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
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
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn deg2rad(&self) -> Result<Array<N>, ArrayError>;

    /// Unwrap by taking the complement of large deltas with respect to the period
    ///
    /// # Arguments
    ///
    /// * `discont` - Maximum discontinuity between values, default is period/2. Values below period/2 are treated as if they were period/2.
    /// * `axis` - Axis along which unwrap will operate, default is the last axis.
    /// * `period` - Size of the range over which the input wraps, default is 2 pi.
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr = Array::flat(vec![0.5, 1.5, 3.2, 6.8, 5.9]).unwrap();
    /// let expected = Array::flat(vec![0.5, 1.5, 3.2, 0.51681469, -0.38318531]);
    /// assert_eq!(format!("{expected:.8?}"), format!("{:.8?}", arr.unwrap_phase(None, None, None)));
    /// ```
    ///
    /// # Errors
    ///
    /// may returns `ArrayError`
    fn unwrap_phase(&self, discont: Option<Array<f64>>, axis: Option<isize>, period: Option<Array<f64>>) -> Result<Array<N>, ArrayError>;
}

impl <N: NumericOps> ArrayTrigonometric<N> for Array<N> {

    fn sin(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().sin()))
    }

    fn cos(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().cos()))
    }

    fn tan(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().tan()))
    }

    fn asin(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().asin()))
    }

    fn acos(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().acos()))
    }

    fn atan(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().atan()))
    }

    fn atan2(&self, other: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().atan2(tuple.1.to_f64())))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn hypot(&self, other: &Self) -> Result<Self, ArrayError> {
        let broadcasted = self.broadcast(other)?;
        let elements = broadcasted.clone().into_iter()
            .map(|tuple| N::from(tuple.0.to_f64().hypot(tuple.1.to_f64())))
            .collect();
        Self::new(elements, broadcasted.get_shape()?)
    }

    fn degrees(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().to_degrees()))
    }

    fn rad2deg(&self) -> Result<Self, ArrayError> {
        self.degrees()
    }

    fn radians(&self) -> Result<Self, ArrayError> {
        self.map(|i| N::from(i.to_f64().to_radians()))
    }

    fn deg2rad(&self) -> Result<Self, ArrayError> {
        self.radians()
    }

    fn unwrap_phase(&self, discont: Option<Array<f64>>, axis: Option<isize>, period: Option<Array<f64>>) -> Result<Self, ArrayError> {

        fn parse_parameter<N: Numeric>(array: &Array<N>, parameter: &Array<N>) -> Result<Array<N>, ArrayError> {
            let self_len = array.len()? - 1;
            parameter.len()?.is_one_of(vec![&1, &self_len])?;
            let result =
                if parameter.len()? == self_len { parameter.clone() }
                else { Array::flat(vec![parameter[0]; self_len])? };
            Ok(result)
        }

        let period = period.unwrap_or(Array::single(std::f64::consts::PI * 2.)?);
        let discont = discont.unwrap_or((period.clone() / 2.)?);
        let (mut discont, mut period) = (discont.to_array_num()?, period.to_array_num()?);

        if self.ndim()? == 1 {
            period = parse_parameter(self, &period)?;
            discont = parse_parameter(self, &discont)?;

            let mut unwrapped_phase = self.get_elements()?;
            unwrapped_phase.clone()
                .windows(2)
                .enumerate()
                .filter(|(idx, window)| N::from((window[1] - window[0]).to_f64().abs()) > discont[*idx])
                .for_each(|(i, _)| {
                    let diff = (unwrapped_phase[i + 1] - unwrapped_phase[i]).to_f64();
                    unwrapped_phase[i + 1..].iter_mut().for_each(|val| *val -= N::from(diff.signum() * ((diff.abs() / period[i].to_f64()).ceil() * period[i].to_f64())));
                });
            Self::new(unwrapped_phase, self.get_shape()?)
        } else {
            let axis = axis.unwrap_or(-1);
            let axis = self.normalize_axis(axis);

            let b_shape = self.get_shape()?.update_at(axis, 1);
            let period = period.broadcast_to(b_shape.clone())?;
            let discont = discont.broadcast_to(b_shape)?;

            let parts = self.get_shape()?.remove_at(axis).into_iter().product();
            let partial = self
                .moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()])?
                .ravel().split(parts, None)?;

            let parameter_parts = period.get_shape()?.remove_at(axis).into_iter().product();
            let period_partial = period
                .moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()])?
                .ravel().split(parameter_parts, None)?;
            let discont_partial = discont
                .moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()])?
                .ravel().split(parameter_parts, None)?;

            let mut results = vec![];
            for i in 0..partial.len() {
                let discont_p = Some(discont_partial[i].to_array_f64()?);
                let period_p = Some(period_partial[i].to_array_f64()?);
                results.push(partial[i].unwrap_phase(discont_p, None, period_p)?);
            };
            let mut tmp_shape = self.get_shape()?;
            let tmp_to_push = tmp_shape.remove(axis);
            tmp_shape.push(tmp_to_push);

            let result = results.into_iter()
                .flatten()
                .collect::<Self>()
                .reshape(&tmp_shape);
            if axis == 0 { result.rollaxis((self.ndim()? - 1).to_isize(), None) }
            else { result.moveaxis(vec![axis.to_isize()], vec![self.ndim()?.to_isize()]) }
                .reshape(&self.get_shape()?)
        }
    }
}

impl <N: NumericOps> ArrayTrigonometric<N> for Result<Array<N>, ArrayError> {

    fn sin(&self) -> Self {
        self.clone()?.sin()
    }

    fn cos(&self) -> Self {
        self.clone()?.cos()
    }

    fn tan(&self) -> Self {
        self.clone()?.tan()
    }

    fn asin(&self) -> Self {
        self.clone()?.asin()
    }

    fn acos(&self) -> Self {
        self.clone()?.acos()
    }

    fn atan(&self) -> Self {
        self.clone()?.atan()
    }

    fn atan2(&self, other: &Array<N>) -> Self {
        self.clone()?.atan2(other)
    }

    fn hypot(&self, other: &Array<N>) -> Self {
        self.clone()?.hypot(other)
    }

    fn degrees(&self) -> Self {
        self.clone()?.degrees()
    }

    fn rad2deg(&self) -> Self {
        self.clone()?.rad2deg()
    }

    fn radians(&self) -> Self {
        self.clone()?.radians()
    }

    fn deg2rad(&self) -> Self {
        self.clone()?.deg2rad()
    }

    fn unwrap_phase(&self, discont: Option<Array<f64>>, axis: Option<isize>, period: Option<Array<f64>>) -> Self {
        self.clone()?.unwrap_phase(discont, axis, period)
    }
}
