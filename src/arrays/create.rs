use crate::arrays::Array;
use crate::traits::{
    create::ArrayCreate,
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

    fn ones(shape: Vec<usize>) -> Self {
        Array::new(vec![N::ONE; shape.iter().product()], shape.clone())
    }

    fn full(shape: Vec<usize>, fill_value: N) -> Self {
        Array::new(vec![fill_value; shape.iter().product()], shape.clone())
    }
}
