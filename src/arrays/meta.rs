use crate::arrays::Array;
use crate::prelude::ArrayError;
use crate::traits::{
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayMeta<N> for Array<N> {

    fn get_elements(&self) -> Vec<N> {
        self.elements.clone()
    }

    fn get_shape(&self) -> Vec<usize> {
        self.shape.clone()
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

impl <N: Numeric> ArrayMeta<N> for Result<Array<N>, ArrayError> {

    fn get_elements(&self) -> Vec<N> {
        self.as_ref().unwrap().elements.clone()
    }

    fn get_shape(&self) -> Vec<usize> {
        self.as_ref().unwrap().shape.clone()
    }

    fn ndim(&self) -> usize {
        self.as_ref().unwrap().shape.len()
    }

    fn len(&self) -> usize {
        self.as_ref().unwrap().elements.len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
