use crate::{
    core::prelude::*,
    errors::prelude::*,
    numeric::prelude::*,
    validators::prelude::*,
};

/// ArrayTrait - Array Create functions
pub trait ArrayCreateNumeric<N: Numeric> where Self: Sized + Clone {

    /// Creates new array with random elements from (0 ..= 1) range
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(4, Array::<f64>::rand(vec![4]).len().unwrap());
    /// assert_eq!(64, Array::<f64>::rand(vec![4, 4, 4]).len().unwrap());
    /// ```
    fn rand(shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new 2d array with ones on the diagonal and zeros elsewhere
    ///
    /// # Arguments
    ///
    /// * `n` - number of rows
    /// * `m` - number of columns. optional, defaulted to n
    /// * `k` - index of diagonal. optional, defaulted to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0]]), Array::<i32>::eye(2, Some(3), Some(0)));
    /// assert_eq!(array!([[0, 1, 0], [0, 0, 1]]), Array::<i32>::eye(2, Some(3), Some(1)));
    /// ```
    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Result<Self, ArrayError>;

    /// Creates new identity 2d array
    ///
    /// # Arguments
    ///
    /// * `n` - numbers of rows and columns of resulting array
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([[1, 0], [0, 1]]), Array::<i32>::identity(2));
    /// assert_eq!(array!([[1, 0, 0], [0, 1, 0], [0, 0, 1]]), Array::<i32>::identity(3));
    /// ```
    fn identity(dim: usize) -> Result<Self, ArrayError>;

    /// Creates new array of zeros
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array elements
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([0, 0, 0, 0]), Array::<f64>::zeros(vec![4]));
    /// ```
    fn zeros(shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new array of zeros like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([0, 0, 0, 0]), Array::<f64>::zeros_like(&array![1, 2, 3, 4].unwrap()));
    /// ```
    fn zeros_like(other: &Self) -> Result<Self, ArrayError>;

    /// Creates new array of ones
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array shape
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([1, 1, 1, 1]), Array::<f64>::ones(vec![4]));
    /// ```
    fn ones(shape: Vec<usize>) -> Result<Self, ArrayError>;

    /// Creates new array of ones like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([1, 1, 1, 1]), Array::<f64>::ones_like(&array![1, 2, 3, 4].unwrap()));
    /// ```
    fn ones_like(other: &Self) -> Result<Self, ArrayError>;

    /// Creates new array of fill_value
    ///
    /// # Arguments
    ///
    /// * `shape` - vector representing array shape
    /// * `fill_value` - value to fill the array with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([2, 2, 2, 2]), Array::<f64>::full(vec![4], 2.));
    /// ```
    fn full(shape: Vec<usize>, fill_value: N) -> Result<Self, ArrayError>;

    /// Creates new array of fill_value like other array
    ///
    /// # Arguments
    ///
    /// * `other` - array defining the shape of new one
    /// * `fill_value` - value to fill the array with
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([2, 2, 2, 2]), Array::<f64>::full_like(&array![1, 2, 3, 4].unwrap(), 2.));
    /// ```
    fn full_like(other: &Self, fill_value: N) -> Result<Self, ArrayError>;

    // ==== from range

    /// Creates new array from range
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `step` - spacing between values. optional, defaults to 1
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([0., 1., 2., 3., 4.]), Array::arange(0., 4., None));
    /// ```
    fn arange(start: N, stop: N, step: Option<N>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([0., 2.5, 5.]), Array::<f64>::linspace(0., 5., Some(3), None));
    /// ```
    fn linspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::linspace_a(&array!([0., 2.]).unwrap(), &array!([4., 6.]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(array!([[0., 2.], [2., 4.], [4., 6.]]).unwrap(), arr);
    /// let arr: Array<f64> = Array::linspace_a(&array!([[0., 2.], [2., 4.]]).unwrap(), &array!([[4., 6.], [6., 8.]]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(array!([[[0., 2.], [2., 4.]], [[2., 4.], [4., 6.]], [[4., 6.], [6., 8.]]]).unwrap(), arr);
    /// ```
    fn linspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced on a log scale over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    /// * `base` - the base of the log space. optional, defaults to 10
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// assert_eq!(array!([1., 316.22776601683796, 100000.]), Array::<f64>::logspace(0., 5., Some(3), None, None));
    /// ```
    fn logspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>, base: Option<usize>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced on a log scale over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    /// * `base` - the base of the log space. optional, defaults to 10
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let arr: Array<f64> = Array::logspace_a(&array!([0., 2.]).unwrap(), &array!([4., 6.]).unwrap(), Some(3), None, None).unwrap();
    /// assert_eq!(array!([[1., 100.], [100., 10000.], [10000., 1000000.]]).unwrap(), arr);
    /// let arr: Array<f64> = Array::logspace_a(&array!([[0., 2.], [2., 4.]]).unwrap(), &array!([[4., 6.], [6., 8.]]).unwrap(), Some(3), None, None).unwrap();
    /// assert_eq!(array!([[[1., 100.], [100., 10000.]], [[100., 10000.], [10000., 1000000.]], [[10000., 1000000.], [1000000., 100000000.]]]).unwrap(), arr);
    /// ```
    fn logspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>, base: Option<&Array<usize>>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced on a log scale (a geometric progression) over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<f64> = array!([1., 10., 100., 1000.]).unwrap();
    /// let arr: Array<f64> = Array::geomspace(1., 1000., Some(4), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

    /// Creates new array of numbers evenly spaced on a log scale (a geometric progression) over a specified interval
    ///
    /// # Arguments
    ///
    /// * `start` - start of interval
    /// * `stop` - end of interval
    /// * `num` - number of samples to generate. optional, defaults to 50
    /// * `endpoint` - determines if `stop` should be included in returned data. optional, defaults to true
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<f64> = array!([[1., 10.], [10., 100.], [100., 1000.], [1000., 10000.]]).unwrap();
    /// let arr: Array<f64> = Array::geomspace_a(&array!([1., 10.]).unwrap(), &array!([1000., 10000.]).unwrap(), Some(4), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// let expected: Array<f64> = array!([[[1., 10.], [10., 100.]], [[10., 100.], [100., 1000.]], [[100., 1000.], [1000., 10000.]]]).unwrap();
    /// let arr: Array<f64> = Array::geomspace_a(&array!([[1., 10.], [10., 100.]]).unwrap(), &array!([[100., 1000.], [1000., 10000.]]).unwrap(), Some(3), None).unwrap();
    /// assert_eq!(format!("{expected:.4}"), format!("{arr:.4}"));
    /// ```
    fn geomspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError>;

    // ==== matrices

    /// Construct an array with ones at and below the given diagonal and zeros elsewhere
    ///
    /// # Arguments
    ///
    /// * `n` - number of rows
    /// * `m` - number of columns. optional, defaults to `n`
    /// * `k` - chosen diagonal. optional, defaults to 0
    ///
    /// # Examples
    ///
    /// ```
    /// use arr_rs::prelude::*;
    ///
    /// let expected: Array<i32> = array!([[1, 0], [1, 1]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::tri(2, Some(2), None).unwrap());
    ///
    /// let expected: Array<i32> = array!([[1, 0, 0], [1, 1, 0], [1, 1, 1]]).unwrap();
    /// assert_eq!(expected, Array::<i32>::tri(3, Some(3), None).unwrap());
    /// ```
    fn tri(n: usize, m: Option<usize>, k: Option<isize>) -> Result<Self, ArrayError>;
}

impl <N: Numeric> ArrayCreateNumeric<N> for Array<N> {

    fn rand(shape: Vec<usize>) -> Result<Self, ArrayError> {
        let size = shape.iter().product();
        let mut elements: Vec<N> = Vec::with_capacity(size);
        (0..size).for_each(|_| elements.push(N::rand(N::zero()..=N::one())));
        Self::new(elements, shape)
    }

    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Result<Self, ArrayError> {
        let m = m.unwrap_or(n);
        let k = k.unwrap_or(0);

        let elements = (0 .. n * m)
            .map(|i| {
                let (row, col) = (i / m, i % m);
                if col >= k && col - k == row { N::one() } else { N::zero() }
            })
            .collect();

        Self::new(elements, vec![n, m])
    }

    fn identity(n: usize) -> Result<Self, ArrayError> {
        let elements = (0 .. n * n)
            .map(|i|
                if i % (n + 1) == 0 { N::one() }
                else { N::zero() })
            .collect();
        Self::new(elements, vec![n, n])
    }

    fn zeros(shape: Vec<usize>) -> Result<Self, ArrayError> {
        Self::new(vec![N::zero(); shape.iter().product()], shape.clone())
    }

    fn zeros_like(other: &Self) -> Result<Self, ArrayError> {
        Self::new(vec![N::zero(); other.get_shape()?.iter().product()], other.get_shape()?)
    }

    fn ones(shape: Vec<usize>) -> Result<Self, ArrayError> {
        Self::new(vec![N::one(); shape.iter().product()], shape.clone())
    }

    fn ones_like(other: &Self) -> Result<Self, ArrayError> {
        Self::new(vec![N::one(); other.get_shape()?.iter().product()], other.get_shape()?)
    }

    fn full(shape: Vec<usize>, fill_value: N) -> Result<Self, ArrayError> {
        Self::new(vec![fill_value; shape.iter().product()], shape.clone())
    }

    fn full_like(other: &Self, fill_value: N) -> Result<Self, ArrayError> {
        Self::new(vec![fill_value; other.get_shape()?.iter().product()], other.get_shape()?)
    }

    // ==== from range

    fn arange(start: N, stop: N, step: Option<N>) -> Result<Self, ArrayError> {
        let step = step.unwrap_or(N::one()).to_f64();
        let size = ((stop.to_f64() + 1. - start.to_f64()) / step).to_usize();
        let mut elements = Vec::with_capacity(size);
        let mut value = start.to_f64();
        for _ in 0 .. size {
            elements.push(value);
            value += step;
        }
        Self::flat(elements.into_iter().map(N::from).collect())
    }

    fn linspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Self, ArrayError> {
        let (num, endpoint) = (num.unwrap_or(50), endpoint.unwrap_or(true));
        let delta = if endpoint { 1 } else { 0 };
        let step = (stop.to_f64() - start.to_f64()) / (num - delta) as f64;

        let result = (0..num)
            .map(|i| start.to_f64() + i as f64 * step).enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { stop.to_f64() } else { val })
            .map(N::from).collect();
        Ok(result)
    }

    fn linspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Array<N>, ArrayError> {
        let start = if start.len()? == 1 { Self::full_like(stop, start[0])? } else { start.clone() };
        let stop = if stop.len()? == 1 { Self::full_like(&start, stop[0])? } else { stop.clone() };
        assert_eq!(start.get_shape(), stop.get_shape());
        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape()?.iter().copied());
        new_shape.reverse();

        let values = start.into_iter().zip(stop)
            .map(|(a, b)| Self::linspace(a, b, num, endpoint).get_elements())
            .collect::<Vec<Result<Vec<N>, ArrayError>>>().has_error()?.iter()
            .map(|a| a.as_ref().unwrap().clone()).collect::<Vec<Vec<N>>>();
        let reshaped = Self::flat(values.into_iter().flatten().collect()).reshape(&new_shape);
        if let Err(error) = reshaped { Err(error) }
        else { reshaped?.transpose(None) }
    }

    fn logspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>, base: Option<usize>) -> Result<Self, ArrayError> {
        let (num, endpoint, base) = (num.unwrap_or(50), endpoint.unwrap_or(true), base.unwrap_or(10).to_f64());
        let delta = if endpoint { 1 } else { 0 };
        let (log_start, log_stop) = (base.powf(start.to_f64()), base.powf(stop.to_f64()));
        let log_step = (log_stop / log_start).powf(1. / (num - delta) as f64);

        let result = (0..num)
            .map(|i| log_start * log_step.powf(i as f64))
            .enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { log_stop } else { val })
            .map(N::from)
            .collect();
        Ok(result)
    }

    fn logspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>, base: Option<&Array<usize>>) -> Result<Array<N>, ArrayError> {
        let start = if start.len()? == 1 { Self::full_like(stop, start[0])? } else { start.clone() };
        let stop = if stop.len()? == 1 { Self::full_like(&start, stop[0])? } else { stop.clone() };
        start.matches_shape(&stop.get_shape()?)?;

        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape()?.iter().copied());
        new_shape.reverse();

        let base = base.unwrap_or(&Array::flat(vec![10])?).clone();
        let base = if base.len()? == 1 { Array::<usize>::full_like(&Array::rand(start.get_shape()?)?, base[0])? } else { base };
        start.matches_shape(&base.get_shape()?)?;

        let values = start.into_iter().zip(stop).zip(base)
            .map(|((a, b), c)| Self::logspace(a, b, num, endpoint, Some(c)).get_elements())
            .collect::<Vec<Result<Vec<N>, ArrayError>>>()
            .has_error()?.iter()
            .map(|a| a.as_ref().unwrap().clone())
            .collect::<Vec<Vec<N>>>();
        let reshaped = Self::flat(values.into_iter().flatten().collect()).reshape(&new_shape);
        if let Err(error) = reshaped { Err(error) }
        else { reshaped.unwrap().transpose(None) }
    }

    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Array<N>, ArrayError> {
        if start == N::zero() {
            return Err(ArrayError::ParameterError { param: "start", message: "geometric sequence cannot include zero" });
        } else if stop == N::zero() {
            return Err(ArrayError::ParameterError { param: "stop", message: "geometric sequence cannot include zero" });
        }

        let (num, endpoint) = (num.unwrap_or(50), endpoint.unwrap_or(true));
        let delta = if endpoint { 1 } else { 0 };
        let ratio = (stop.to_f64() / start.to_f64()).to_f64().powf(1.0 / (num - delta) as f64);

        let result = (0..num)
            .map(|i| start.to_f64() * ratio.powf(i as f64))
            .enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { stop.to_f64() } else { val })
            .map(N::from)
            .collect();
        Ok(result)
    }

    fn geomspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Result<Array<N>, ArrayError> {
        let start = if start.len()? == 1 { Self::full_like(stop, start[0])? } else { start.clone() };
        let stop = if stop.len()? == 1 { Self::full_like(&start, stop[0])? } else { stop.clone() };
        start.matches_shape(&stop.get_shape()?)?;

        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape()?.iter().copied());
        new_shape.reverse();

        let values = start.into_iter().zip(stop)
            .map(|(a, b)| Self::geomspace(a, b, num, endpoint))
            .collect::<Vec<Result<Array<N>, _>>>()
            .has_error()?.iter()
            .map(|a| a.get_elements()).collect::<Vec<Result<Vec<N>, ArrayError>>>()
            .has_error()?.into_iter()
            .map(|a| a.unwrap())
            .collect::<Vec<Vec<N>>>();
        Self::flat(values.into_iter().flatten().collect())
            .reshape(&new_shape)?
            .transpose(None)
    }

    // ==== matrices

    fn tri(n: usize, m: Option<usize>, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        let (m, k) = (m.unwrap_or(n), k.unwrap_or(0));

        let elements = (0..n)
            .flat_map(|i| (0..m).map(move |j|
                if j as isize <= i as isize + k { N::one() }
                else { N::zero() }
            ))
            .collect();

        Self::new(elements, vec![n, m])
    }
}