use crate::arrays::base::ArrayBase;

#[derive(Clone, Debug)]
pub struct Array {
    pub(crate) elements: Vec<i32>,
    pub(crate) shape: Vec<usize>,
}

impl ArrayBase for Array {

    fn new(elements: Vec<i32>, shape: Vec<usize>) -> Self {
        assert_eq!(elements.len(), shape.iter().product(), "Shape must match values length");
        Array { elements, shape, }
    }

    fn empty() -> Self {
        Array::new(vec![], vec![0])
    }

    fn zeros(shape: Vec<usize>) -> Self {
        Array::new(vec![0; shape.iter().product()], shape.clone())
    }

    fn ones(shape: Vec<usize>) -> Self {
        Array::new(vec![1; shape.iter().product()], shape.clone())
    }

    fn product(&self) -> i32 {
        self.elements.iter().product()
    }

    fn sum(&self) -> i32 {
        self.elements.iter().sum()
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
}
