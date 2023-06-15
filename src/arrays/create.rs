use crate::arrays::Array;
use crate::traits::{
    create::{
        ArrayCreate,
        ArrayCreateFrom,
        ArrayCreateNumeric,
    },
    errors::ArrayError,
    manipulate::{
        ArrayManipulate,
        axis::ArrayAxis,
    },
    meta::ArrayMeta,
    types::{
        ArrayElement,
        numeric::Numeric,
    },
    validators::{
        validate_dimension::ValidateDimension,
        validate_has_error::ValidateHasError,
        validate_shape::ValidateShape,
    },
};

impl <T: ArrayElement> ArrayCreate<T> for Array<T> {

    // ==== from data

    fn new(elements: Vec<T>, shape: Vec<usize>) -> Result<Array<T>, ArrayError> {
        shape.matches_values_len(&elements)?;
        Ok(Self { elements, shape, })
    }

    fn single(element: T) -> Result<Self, ArrayError> {
        Self::new(vec![element], vec![1])
    }

    fn flat(elements: Vec<T>) -> Result<Self, ArrayError> {
        Self::new(elements.clone(), vec![elements.len()])
    }

    fn empty() -> Result<Self, ArrayError> {
        Self::new(vec![], vec![0])
    }
}

impl <N: ArrayElement + Numeric> ArrayCreateNumeric<N> for Array<N> {

    fn rand(shape: Vec<usize>) -> Result<Self, ArrayError> {
        let size = shape.iter().product();
        let mut elements: Vec<N> = Vec::with_capacity(size);
        (0..size).for_each(|_| elements.push(N::rand(N::ZERO..=N::ONE)));
        Self::new(elements, shape)
    }

    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Result<Self, ArrayError> {
        let m = m.unwrap_or(n);
        let k = k.unwrap_or(0);

        let elements = (0 .. n * m)
            .map(|i| {
                let (row, col) = (i / m, i % m);
                if col >= k && col - k == row { N::ONE } else { N::ZERO }
            })
            .collect();

        Self::new(elements, vec![n, m])
    }

    fn identity(n: usize) -> Result<Self, ArrayError> {
        let elements = (0 .. n * n)
            .map(|i|
                if i % (n + 1) == 0 { N::ONE }
                else { N::ZERO })
            .collect();
        Self::new(elements, vec![n, n])
    }

    fn zeros(shape: Vec<usize>) -> Result<Self, ArrayError> {
        Self::new(vec![N::ZERO; shape.iter().product()], shape.clone())
    }

    fn zeros_like(other: &Self) -> Result<Self, ArrayError> {
        Self::new(vec![N::ZERO; other.get_shape()?.iter().product()], other.get_shape()?)
    }

    fn ones(shape: Vec<usize>) -> Result<Self, ArrayError> {
        Self::new(vec![N::ONE; shape.iter().product()], shape.clone())
    }

    fn ones_like(other: &Self) -> Result<Self, ArrayError> {
        Self::new(vec![N::ONE; other.get_shape()?.iter().product()], other.get_shape()?)
    }

    fn full(shape: Vec<usize>, fill_value: N) -> Result<Self, ArrayError> {
        Self::new(vec![fill_value; shape.iter().product()], shape.clone())
    }

    fn full_like(other: &Self, fill_value: N) -> Result<Self, ArrayError> {
        Self::new(vec![fill_value; other.get_shape()?.iter().product()], other.get_shape()?)
    }

    // ==== from range

    fn arange(start: N, stop: N, step: Option<N>) -> Result<Self, ArrayError> {
        let step = step.unwrap_or(N::ONE).to_f64();
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
        let reshaped = Self::flat(values.into_iter().flatten().collect()).reshape(new_shape);
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
        let reshaped = Self::flat(values.into_iter().flatten().collect()).reshape(new_shape);
        if let Err(error) = reshaped { Err(error) }
        else { reshaped.unwrap().transpose(None) }
    }

    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Result<Array<N>, ArrayError> {
        if start == N::ZERO {
            return Err(ArrayError::ParameterError { param: "start", message: "geometric sequence cannot include zero" });
        } else if stop == N::ZERO {
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
            .reshape(new_shape)?
            .transpose(None)
    }

    // ==== matrices

    fn tri(n: usize, m: Option<usize>, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        let (m, k) = (m.unwrap_or(n), k.unwrap_or(0));

        let elements = (0..n)
            .flat_map(|i| (0..m).map(move |j|
                if j as isize <= i as isize + k { N::ONE }
                else { N::ZERO }
            ))
            .collect();

        Self::new(elements, vec![n, m])
    }
}

impl <N: Numeric> ArrayCreateFrom<N> for Array<N> {

    // ==== matrices

    fn diag(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.is_dim_supported(&[1, 2])?;

        fn diag_1d<N: ArrayElement + Numeric>(data: &Array<N>, k: isize) -> Result<Array<N>, ArrayError> {
            let size = data.get_shape()?[0];
            let abs_k = k.unsigned_abs();
            let new_shape = vec![size + abs_k, size + abs_k];
            let data_elements = data.get_elements()?;
            let elements = (0..new_shape[0] * new_shape[1])
                .map(|idx| {
                    let (i, j) = (idx / new_shape[1], idx % new_shape[1]);
                    if k >= 0 && j == i + k as usize {
                        if i < size { data_elements[i] }
                        else { N::ZERO }
                    } else if k < 0 && i == j + abs_k {
                        if j < size { data_elements[j] }
                        else { N::ZERO }
                    } else {
                        N::ZERO
                    }
                })
                .collect();

            Array::new(elements, new_shape)
        }

        fn diag_2d<N: ArrayElement + Numeric>(data: &Array<N>, k: isize) -> Result<Array<N>, ArrayError> {
            let rows = data.get_shape()?[0];
            let cols = data.get_shape()?[1];
            let (start_row, start_col) =
                if k >= 0 { (0, k as usize) }
                else { ((-k) as usize, 0) };

            let data_elements = data.get_elements()?;
            let elements = (start_row..rows)
                .zip(start_col..cols)
                .map(|(i, j)| data_elements[i * cols + j])
                .collect::<Vec<N>>();

            Array::new(elements.clone(), vec![elements.len()])
        }

        let k = k.unwrap_or(0);
        if self.ndim()? == 1 { diag_1d(self, k) }
        else { diag_2d(self, k) }
    }

    fn diagflat(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.ravel()?.diag(k)
    }

    fn tril(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        let k = k.unwrap_or(0);
        self.apply_triangular(k, |j, i, k| j > i + k)
    }

    fn triu(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        let k = k.unwrap_or(0);
        self.apply_triangular(k, |j, i, k| j < i + k)
    }

    fn vander(&self, n: Option<usize>, increasing: Option<bool>) -> Result<Array<N>, ArrayError> {
        self.is_dim_supported(&[1])?;

        let size = self.shape[0];
        let increasing = increasing.unwrap_or(false);
        let n_columns = n.unwrap_or(size);
        let mut elements = Vec::with_capacity(size * n_columns);

        for item in self.into_iter() {
            for i in 0..n_columns {
                let power = if increasing { i } else { n_columns - i - 1 } as f64;
                elements.push(N::from(item.to_f64().powf(power)));
            }
        }

        Self::new(elements, vec![size, n_columns])
    }
}

impl <N: ArrayElement + Numeric> ArrayCreateFrom<N> for Result<Array<N>, ArrayError> {

    fn diag(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.diag(k)
    }

    fn diagflat(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.diagflat(k)
    }

    fn tril(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.tril(k)
    }

    fn triu(&self, k: Option<isize>) -> Result<Array<N>, ArrayError> {
        self.clone()?.triu(k)
    }

    fn vander(&self, n: Option<usize>, increasing: Option<bool>) -> Result<Array<N>, ArrayError> {
        self.clone()?.vander(n, increasing)
    }
}

impl <N: Numeric> Array<N> {

    fn apply_triangular<F>(&self, k: isize, compare: F) -> Result<Array<N>, ArrayError>
        where F: Fn(isize, isize, isize) -> bool {
        let last_dim = self.shape.len() - 1;
        let second_last_dim = self.shape.len() - 2;
        let chunk_size = self.shape[last_dim] * self.shape[second_last_dim];

        let elements = self.elements
            .chunks(chunk_size)
            .flat_map(|chunk| {
                chunk
                    .iter()
                    .enumerate()
                    .map(|(idx, &value)| {
                        let i = (idx / self.shape[last_dim]) % self.shape[second_last_dim];
                        let j = idx % self.shape[last_dim];
                        if compare(j as isize, i as isize, k) { N::ZERO } else { value }
                    })
            })
            .collect();

        Self::new(elements, self.shape.clone())
    }
}
