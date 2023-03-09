use num_traits::NumCast;

use crate::base::base_array::ArrayBase;

/// default Array implementation
#[derive(Clone, Debug)]
pub struct Array {
    pub(crate) elements: Vec<f64>,
    pub(crate) shape: Vec<usize>,
}

impl ArrayBase for Array {

    fn new<I, A>(elements: I, shape: Vec<usize>) -> Self
    where
        A: NumCast,
        I: IntoIterator<Item=A> + Clone {
        assert_eq!(elements.clone().into_iter().count(), shape.iter().product(), "Shape must match values length");
        let elems = elements.into_iter().map(|i| i.to_f64().unwrap()).collect();
        Array { elements: elems, shape, }
    }

    fn empty() -> Self {
        Array::new(Vec::<f64>::new(), vec![0])
    }

    fn zeros(shape: Vec<usize>) -> Self {
        Array::new(vec![0; shape.iter().product()], shape.clone())
    }

    fn ones(shape: Vec<usize>) -> Self {
        Array::new(vec![1; shape.iter().product()], shape.clone())
    }

    fn product<F>(&self) -> F
        where F: NumCast {
        NumCast::from(self.elements.iter().product::<f64>()).unwrap()
    }

    fn sum<F>(&self) -> F
        where F: NumCast {
        NumCast::from(self.elements.iter().sum::<f64>()).unwrap()
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

    fn get_elements<F>(&self) -> Vec<F> where F: NumCast {
        self.elements.clone().into_iter().map(|m| NumCast::from(m).unwrap()).collect()
    }

    fn get_shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    fn ravel(&self) -> Self {
        Array::new(self.elements.clone(), vec![self.len()])
    }
}
