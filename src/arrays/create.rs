use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
    meta::ArrayMeta,
    types::Numeric,
};

impl <N: Numeric> ArrayCreate<N> for Array<N> {

    fn new(elements: Vec<N>, shape: Vec<usize>) -> Self {
        assert_eq!(elements.len(), shape.iter().product(), "Shape must match values length");
        Array { elements, shape, }
    }

    fn flat(elements: Vec<N>) -> Self {
        Array { elements: elements.clone(), shape: vec![elements.len()], }
    }

    fn rand(shape: Vec<usize>) -> Self {
        let size = shape.iter().product();
        let mut elements: Vec<N> = Vec::with_capacity(size);
        (0..size).for_each(|_| elements.push(N::rand(N::ZERO..=N::ONE)));
        Array { elements, shape }
    }

    fn empty() -> Self {
        Array::new(Vec::<N>::new(), vec![0])
    }

    fn eye(n: usize, m: Option<usize>, k: Option<usize>) -> Self {
        let m = m.unwrap_or(n);
        let k = k.unwrap_or(0);

        let elements = (0 .. n * m)
            .map(|i| {
                let (row, col) = (i / m, i % m);
                if col >= k && col - k == row { N::ONE } else { N::ZERO }
            })
            .collect();

        Array { elements, shape: vec![n, m] }
    }

    fn identity(n: usize) -> Self {
        let elements = (0 .. n * n)
            .map(|i|
                if i % (n + 1) == 0 { N::ONE }
                else { N::ZERO })
            .collect();
        Array { elements, shape: vec![n, n] }
    }

    fn zeros(shape: Vec<usize>) -> Self {
        Array::new(vec![N::ZERO; shape.iter().product()], shape.clone())
    }

    fn zeros_like(other: &Self) -> Self {
        Array::new(vec![N::ZERO; other.get_shape().iter().product()], other.get_shape())
    }

    fn ones(shape: Vec<usize>) -> Self {
        Array::new(vec![N::ONE; shape.iter().product()], shape.clone())
    }

    fn ones_like(other: &Self) -> Self {
        Array::new(vec![N::ONE; other.get_shape().iter().product()], other.get_shape())
    }

    fn full(shape: Vec<usize>, fill_value: N) -> Self {
        Array::new(vec![fill_value; shape.iter().product()], shape.clone())
    }

    fn full_like(other: &Self, fill_value: N) -> Self {
        Array::new(vec![fill_value; other.get_shape().iter().product()], other.get_shape())
    }

    fn arange(start: N, stop: N, step: Option<N>) -> Self {
        let step = step.unwrap_or(N::ONE).to_f64();
        let size = ((stop.to_f64() - start.to_f64()) / step).to_usize();
        let mut elements = Vec::with_capacity(size);
        let mut value = start.to_f64();
        for _ in 0..size {
            elements.push(value);
            value += step;
        }
        Array::new(elements.into_iter().map(N::from).collect(), vec![size])
    }

    fn linspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Self {
        let (num, endpoint) = (num.unwrap_or(50), endpoint.unwrap_or(true));
        let delta = if endpoint { 1 } else { 0 };
        let step = (stop.to_f64() - start.to_f64()) / (num - delta) as f64;

        (0..num)
            .map(|i| start.to_f64() + i as f64 * step).enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { stop.to_f64() } else { val })
            .map(N::from).collect()
    }

    fn linspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Self {
        let start = if start.len() == 1 { Array::full_like(stop, start[0]) } else { start.clone() };
        let stop = if stop.len() == 1 { Array::full_like(&start, stop[0]) } else { stop.clone() };
        assert_eq!(start.get_shape(), stop.get_shape());
        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape().iter().copied());

        let values = start.into_iter().zip(stop)
            .map(|(a, b)| Self::linspace(a, b, num, endpoint).get_elements())
            .collect::<Vec<Vec<N>>>();
        let output = (0 .. values[0].len())
            .map(|i| (0 .. values.len())
                .map(|j| values[j][i])
                .collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
            .into_iter().flatten().collect();
        Array::new(output, new_shape)
    }

    fn logspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>, base: Option<usize>) -> Self {
        let (num, endpoint, base) = (num.unwrap_or(50), endpoint.unwrap_or(true), base.unwrap_or(10).to_f64());
        let delta = if endpoint { 1 } else { 0 };
        let (log_start, log_stop) = (base.powf(start.to_f64()), base.powf(stop.to_f64()));
        let log_step = (log_stop / log_start).powf(1. / (num - delta) as f64);

        (0..num)
            .map(|i| log_start * log_step.powf(i as f64))
            .enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { log_stop } else { val })
            .map(N::from)
            .collect()
    }

    fn logspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>, base: Option<&Array<usize>>) -> Self {
        let start = if start.len() == 1 { Array::full_like(stop, start[0]) } else { start.clone() };
        let stop = if stop.len() == 1 { Array::full_like(&start, stop[0]) } else { stop.clone() };
        assert_eq!(start.get_shape(), stop.get_shape());
        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape().iter().copied());

        let base = base.unwrap_or(&Array::flat(vec![10])).clone();
        let base = if base.len() == 1 { Array::<usize>::full_like(&Array::rand(start.get_shape()), base[0]) } else { base };
        assert_eq!(start.get_shape(), base.get_shape());

        let values = start.into_iter().zip(stop).zip(base)
            .map(|((a, b), c)| Self::logspace(a, b, num, endpoint, Some(c)).get_elements())
            .collect::<Vec<Vec<N>>>();
        let output = (0 .. values[0].len())
            .map(|i| (0 .. values.len())
                .map(|j| values[j][i])
                .collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
            .into_iter().flatten().collect();
        Array::new(output, new_shape)
    }

    fn geomspace(start: N, stop: N, num: Option<usize>, endpoint: Option<bool>) -> Self {
        assert!(start != N::ZERO && stop != N::ZERO, "Geometric sequence cannot include zero");
        let (num, endpoint) = (num.unwrap_or(50), endpoint.unwrap_or(true));
        let delta = if endpoint { 1 } else { 0 };
        let ratio = (stop.to_f64() / start.to_f64()).to_f64().powf(1.0 / (num - delta) as f64);

        (0..num)
            .map(|i| start.to_f64() * ratio.powf(i as f64))
            .enumerate()
            .map(|(i, val)| if endpoint && i == num - 1 { stop.to_f64() } else { val })
            .map(N::from)
            .collect()
    }

    fn geomspace_a(start: &Self, stop: &Self, num: Option<usize>, endpoint: Option<bool>) -> Self {
        let start = if start.len() == 1 { Array::full_like(stop, start[0]) } else { start.clone() };
        let stop = if stop.len() == 1 { Array::full_like(&start, stop[0]) } else { stop.clone() };
        assert_eq!(start.get_shape(), stop.get_shape());
        let mut new_shape = vec![num.unwrap_or(50)];
        new_shape.extend(start.get_shape().iter().copied());

        let values = start.into_iter().zip(stop)
            .map(|(a, b)| Self::geomspace(a, b, num, endpoint).get_elements())
            .collect::<Vec<Vec<N>>>();
        let output = (0 .. values[0].len())
            .map(|i| (0 .. values.len())
                .map(|j| values[j][i])
                .collect::<Vec<_>>())
            .collect::<Vec<Vec<_>>>()
            .into_iter().flatten().collect();
        Array::new(output, new_shape)
    }
}
