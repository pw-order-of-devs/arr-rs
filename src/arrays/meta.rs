use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    meta::ArrayMeta,
    types::numeric::Numeric,
};

impl <N: Numeric> ArrayMeta<N> for Array<N> {

    fn get_elements(&self) -> Result<Vec<N>, ArrayError> {
        Ok(self.elements.clone())
    }

    fn get_shape(&self) -> Result<Vec<usize>, ArrayError> {
        Ok(self.shape.clone())
    }

    fn ndim(&self) -> Result<usize, ArrayError> {
        Ok(self.shape.len())
    }

    fn len(&self) -> Result<usize, ArrayError> {
        Ok(self.elements.len())
    }

    fn is_empty(&self) -> Result<bool, ArrayError> {
        Ok(self.len()? == 0)
    }
}

impl <N: Numeric> ArrayMeta<N> for Result<Array<N>, ArrayError> {

    fn get_elements(&self) -> Result<Vec<N>, ArrayError> {
        self.clone()?.get_elements()
    }

    fn get_shape(&self) -> Result<Vec<usize>, ArrayError> {
        self.clone()?.get_shape()
    }

    fn ndim(&self) -> Result<usize, ArrayError> {
        self.clone()?.ndim()
    }

    fn len(&self) -> Result<usize, ArrayError> {
        self.clone()?.len()
    }

    fn is_empty(&self) -> Result<bool, ArrayError> {
        self.clone()?.is_empty()
    }
}
