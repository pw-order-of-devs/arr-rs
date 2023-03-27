use crate::base::base_array::ArrayBase;
use crate::base::base_type::Numeric;

/// default Array implementation
#[derive(Clone, Debug)]
pub struct Array<N: Numeric> {
    pub(crate) elements: Vec<N>,
    pub(crate) shape: Vec<usize>,
}

impl <N: Numeric> ArrayBase<N> for Array<N> {

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
        (0..size).for_each(|_| elements.push(N::rand(N::ZERO ..= N::ONE)));
        Array { elements, shape }
    }

    fn empty() -> Self {
        Array::new(Vec::<N>::new(), vec![0])
    }

    fn zeros(shape: Vec<usize>) -> Self {
        Array::new(vec![N::ZERO; shape.iter().product()], shape.clone())
    }

    fn ones(shape: Vec<usize>) -> Self {
        Array::new(vec![N::ONE; shape.iter().product()], shape.clone())
    }

    fn reshape(&self, shape: Vec<usize>) -> Self {
        assert_eq!(self.elements.len(), shape.iter().product(), "Shape must match values length");
        Array::new(self.elements.clone(), shape)
    }

    fn product(&self) -> N {
        self.elements.iter().fold(N::ONE, |acc, x| acc * *x)
    }

    fn sum(&self) -> N {
        self.elements.iter().fold(N::ZERO, |acc, x| acc + *x)
    }

    fn ndim(&self) -> usize {
        self.shape.len()
    }

    fn len(&self) -> usize {
        self.elements.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn get_elements(&self) -> Vec<N> {
        self.elements.clone()
    }

    fn get_shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    fn index(&self, coords: &[usize]) -> usize {
        assert_eq!(self.shape.len(), coords.len(), "coords length must match array dimension");
        for (i, _) in coords.iter().enumerate() { assert!(coords[i] < self.shape[i], "coord value must match array shape"); }
        let mut index = 0;
        let mut stride = 1;
        for i in (0..self.shape.len()).rev() {
            index += coords[i] * stride;
            stride *= self.shape[i];
        }
        index
    }

    fn at(&self, coords: &[usize]) -> N {
        self.elements[self.index(coords)]
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Self {
        if range.start > range.end || range.end > self.elements.len() {
            panic!("Slice range out of bounds");
        }
        if self.shape.len() == 1 {
            Array::flat(self.elements[range].into())
        } else if range.len() >= self.shape[0] {
            self.clone()
        } else {
            let new_shape =
                if range.len() > 1 { vec![range.len()].into_iter().chain(self.shape[1..].iter().cloned()).collect() }
                else { self.shape[1..].to_vec() };

            let items: usize = new_shape.iter().product();
            let stride = items / new_shape[0];
            let start_index = new_shape[0] * range.start;

            let mut new_elements = Vec::with_capacity(items);
            for idx in (start_index..start_index + items).step_by(stride) {
                new_elements.extend_from_slice(&self.elements[idx..idx + stride]);
            }
            Array::new(new_elements, new_shape)
        }
    }

    fn ravel(&self) -> Self {
        Array::new(self.elements.clone(), vec![self.len()])
    }

    fn for_each<F: FnMut(&N)>(&self, f: F) {
        self.elements.iter()
            .for_each(f)
    }

    fn for_each_e<F: FnMut(usize, &N)>(&self, mut f: F) {
        self.elements.iter().enumerate()
            .for_each(|(idx, item)| f(idx, item))
    }

    fn map<F: FnMut(&N) -> N>(&self, f: F) -> Self {
        self.elements.iter()
            .map(f)
            .collect()
    }

    fn map_e<F: FnMut(usize, &N) -> N>(&self, mut f: F) -> Self {
        self.elements.iter().enumerate()
            .map(|(idx, item)| f(idx, item))
            .collect()
    }

    fn filter<F: FnMut(&N) -> bool>(&self, mut f: F) -> Self {
        self.elements.clone().into_iter()
            .filter(|item| f(item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_e<F: FnMut(usize, &N) -> bool>(&self, mut f: F) -> Self {
        self.elements.clone().into_iter().enumerate()
            .filter(|(idx, item)| f(*idx, item))
            .map(|i| i.1)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map<F: FnMut(&N) -> Option<N>>(&self, f: F) -> Self {
        self.elements.clone().iter()
            .filter_map(f)
            .collect::<Array<N>>()
            .ravel()
    }

    fn filter_map_e<F: FnMut(usize, &N) -> Option<N>>(&self, mut f: F) -> Self {
        self.elements.clone().iter().enumerate()
            .filter_map(|(idx, item)| f(idx, item))
            .collect::<Array<N>>()
            .ravel()
    }

    fn fold<F: FnMut(&N, &N) -> N>(&self, init: N, mut f: F) -> N {
        self.elements.iter().fold(init, |a, b| f(&a, b))
    }
}
