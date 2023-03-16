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
