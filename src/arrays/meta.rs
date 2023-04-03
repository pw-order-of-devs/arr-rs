use crate::arrays::Array;
use crate::traits::{
    meta::ArrayMeta,
    types::Numeric,
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
