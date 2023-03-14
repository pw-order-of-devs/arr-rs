use crate::base::base_array::ArrayBase;
use crate::base::base_type::Numeric;

/// default Array implementation
#[derive(Clone, Debug)]
pub struct Array<N: Numeric> {
    pub(crate) elements: Vec<N>,
    pub(crate) shape: Vec<usize>,
}

impl <N: Numeric>ArrayBase<N> for Array<N> {

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

    fn ravel(&self) -> Self {
        Array::new(self.elements.clone(), vec![self.len()])
    }
}
