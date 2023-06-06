use crate::arrays::Array;
use crate::traits::{
    errors::ArrayError,
    binary::ArrayBinary,
    create::ArrayCreate,
    meta::ArrayMeta,
    types::numeric::Numeric,
    validators::validate_shape::ValidateShape,
};

impl <N: Numeric> ArrayBinary<N> for Array<N> {

    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.into_iter().zip(other)
            .map(|(&a, &b)| a.bitwise_and(&b))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.into_iter().zip(other)
            .map(|(&a, &b)| a.bitwise_or(&b))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.get_shape()?.is_broadcastable(&other.get_shape()?)?;
        let elements = self.into_iter().zip(other)
            .map(|(&a, &b)| a.bitwise_xor(&b))
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        let elements = self.into_iter()
            .map(|&a| a.bitwise_not())
            .collect();
        Array::new(elements, self.get_shape()?)
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.bitwise_not()
    }
}

impl <N: Numeric> ArrayBinary<N> for Result<Array<N>, ArrayError> {

    fn bitwise_and(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_and(other)
    }

    fn bitwise_or(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_or(other)
    }

    fn bitwise_xor(&self, other: &Array<N>) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_xor(other)
    }

    fn bitwise_not(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.bitwise_not()
    }

    fn invert(&self) -> Result<Array<N>, ArrayError> {
        self.clone()?.invert()
    }
}
